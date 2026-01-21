// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// SPDX-FileCopyrightText: 2024 X-Software GmbH <opensource@x-software.com>

use anyhow::{Context, Result};
use log::*;
use std::{path::Path, process::Command};

use crate::license_info::LicenseInfo;
use crate::settings;
use crate::{
    file_info::FileInfo,
    license_detector::{LicenseDetector, LicenseFile},
};

const RPM_EXECUTABLE: &str = "rpm";

#[derive(Debug)]
struct PackageDB {
    packages: Vec<Package>,
}

impl PackageDB {
    pub fn new() -> Result<Self> {
        let list_packages_args = vec![
            "--query",
            "--all",
            "--queryformat",
            "%{NAME}\\n%{LICENSE}\\n%{VERSION}\\n%{URL}\\n",
        ];
        let output = Command::new(RPM_EXECUTABLE)
            .args(&list_packages_args)
            .output()
            .context(format!(
                "Cannot create command '{} {}'",
                RPM_EXECUTABLE,
                list_packages_args.join(" ")
            ))?;
        if !output.status.success() {
            let error = String::from_utf8(output.stderr)?;
            return Err(anyhow::anyhow!(
                "Execution of '{} {}' failed",
                RPM_EXECUTABLE,
                list_packages_args.join(" ")
            )
            .context(error));
        }
        let stdout = String::from_utf8(output.stdout)?;
        let mut lines = stdout.lines();
        let mut packages = Vec::new();
        loop {
            let Some(name) = lines.next() else {
                break;
            };
            let license = lines
                .next()
                .and_then(|line| if line.is_empty() { None } else { Some(line.to_string()) });
            let version = lines
                .next()
                .and_then(|line| if line.is_empty() { None } else { Some(line.to_string()) });
            let url = lines
                .next()
                .and_then(|line| if line.is_empty() { None } else { Some(line.to_string()) });

            // Query the package files
            let list_packages_args = vec!["-ql", name];
            let output = Command::new(RPM_EXECUTABLE)
                .args(&list_packages_args)
                .output()
                .context(format!(
                    "Cannot create command '{} {}'",
                    RPM_EXECUTABLE,
                    list_packages_args.join(" ")
                ))?;
            if !output.status.success() {
                let error = String::from_utf8(output.stderr)?;
                return Err(anyhow::anyhow!(
                    "Execution of '{} {}' failed",
                    RPM_EXECUTABLE,
                    list_packages_args.join(" ")
                )
                .context(error)
                .context(format!("Cannot get package content of package '{name}'")));
            }
            let files = String::from_utf8(output.stdout)?
                .lines()
                .map(String::from)
                .collect::<Vec<_>>();

            packages.push(Package {
                name: name.to_string(),
                license,
                version,
                url,
                files,
            });
        }

        Ok(Self { packages })
    }

    pub fn query_by_library_name(&self, library_name: &str) -> Result<Package> {
        let packages = self
            .packages
            .iter()
            .filter(|pkg| pkg.files.iter().any(|file| file.contains(library_name)))
            .collect::<Vec<_>>();
        if packages.len() > 1 {
            return Err(anyhow::anyhow!(
                "Cannot find unique package containing the library '{library_name}', packages found: {}",
                packages
                    .iter()
                    .map(|pkg| format!("'{}'", pkg.name))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        if let Some(package) = packages.first() {
            return Ok((*package).clone());
        }
        Err(anyhow::anyhow!(
            "Cannot find any package containing the library '{library_name}'"
        ))
    }
}

#[derive(Debug, Clone)]
pub struct Package {
    name: String,
    version: Option<String>,
    license: Option<String>,
    url: Option<String>,
    files: Vec<String>,
}

impl Package {
    pub fn name(&self) -> &str {
        &self.name
    }

    fn collect_files(&self, patterns: Vec<&str>) -> Result<Vec<String>> {
        let mut found_files: Vec<_> = vec![];
        for pkg_file in &self.files {
            if patterns.iter().any(|p| pkg_file.contains(p)) {
                let path = Path::new(pkg_file).to_path_buf();
                if path.is_file() {
                    found_files.push(path.to_string_lossy().into_owned());
                } else {
                    return Err(anyhow::anyhow!(
                        "The installed file '{}' of package '{}' is missing in the file system",
                        path.to_string_lossy().into_owned(),
                        self.name
                    ));
                }
            }
        }
        Ok(found_files)
    }

    pub fn license_info(&self, lib_info: FileInfo, overrides: &[settings::Override]) -> Result<LicenseInfo> {
        let override_info = settings::Override::find_override(&self.name, overrides);

        let license = {
            if override_info.is_some_and(|x| x.license_id.is_some()) {
                override_info.unwrap().license_id.clone()
            } else {
                self.license.clone()
            }
        };

        let license = if let Some(license) = license {
            license
        } else {
            return Err(anyhow::anyhow!(
                "Missing license identifier for RPM package '{}'",
                self.name
            ));
        };

        let license_file_patterns = vec!["COPY", "LICENSE", "License"];
        let license_files: Vec<_> = if override_info.is_some_and(|x| !x.license_files.is_empty()) {
            override_info
                .unwrap()
                .license_files
                .iter()
                .map(|license_file| LicenseFile {
                    id: license_file.id.clone(),
                    file: license_file.file.clone(),
                })
                .collect()
        } else {
            self.collect_files(license_file_patterns)?
                .iter()
                .map(|file| LicenseFile {
                    id: None,
                    file: file.clone(),
                })
                .collect()
        };

        // Create SPDX license expression from the license IDs:
        let license_expression = if !license.is_empty() {
            match spdx::Expression::parse_mode(license.as_str(), spdx::ParseMode::LAX)
                .context(format!("Cannot parse license expression for package '{}'", self.name))
            {
                Ok(expr) => Some(expr),
                Err(error) => {
                    warn!("{error:?}");
                    None
                }
            }
        } else {
            None
        };

        let license_ids = if let Some(expr) = &license_expression {
            let mut collection = vec![];
            expr.iter()
                .map(|l| match l {
                    spdx::expression::ExprNode::Req(req) => Some(req),
                    spdx::expression::ExprNode::Op(_op) => None,
                })
                .for_each(|expr| {
                    if let Some(expr) = expr
                        && let Some(license) = &expr.req.license.id()
                    {
                        collection.push(*license);
                    }
                });
            collection
        } else {
            vec![]
        };

        // Detect license ids of license texts:
        let license_texts = LicenseDetector::instance().detect_licenses(
            &self.name,
            license_ids.as_slice(),
            license_files.as_slice(),
        )?;

        Ok(LicenseInfo {
            lib_info,
            package_name: self.name.clone(),
            license,
            license_expression,
            license_texts,
            version: self.version.clone(),
            url: self.url.clone(),
        })
    }
}

fn static_package_db() -> Result<&'static PackageDB> {
    static PACKAGE_LIST: std::sync::OnceLock<Result<PackageDB>> = std::sync::OnceLock::new();

    match PACKAGE_LIST.get_or_init(PackageDB::new) {
        Ok(packages) => Ok(packages),
        Err(err) => {
            anyhow::bail!("Failed to initialize: {:?}", err);
        }
    }
}

pub fn package_of_lib(library_name: &str) -> Result<Package> {
    let db = static_package_db()?;
    db.query_by_library_name(library_name)
}
