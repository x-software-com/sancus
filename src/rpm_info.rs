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
const RPM_QUERY_LICENSE: &str = "LICENSE";
const RPM_QUERY_VERSION: &str = "VERSION";
const RPM_QUERY_URL: &str = "URL";

fn package_collect_files(package: &str, patterns: Vec<&str>) -> Result<Vec<String>> {
    let mut found_files: Vec<_> = vec![];

    let list_packages_args = vec!["-ql", package];

    let output = Command::new(RPM_EXECUTABLE)
        .args(list_packages_args.clone())
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
        .context(format!("Cannot get package content of {package}")));
    }
    let stdout = String::from_utf8(output.stdout)?;
    for file in stdout.lines() {
        if patterns.iter().any(|p| file.contains(p)) {
            let path = Path::new(file).to_path_buf();
            if path.is_file() {
                found_files.push(path.to_string_lossy().into_owned());
            } else {
                return Err(anyhow::anyhow!(
                    "The installed file {} of package {package} is missing in the file system",
                    path.to_string_lossy().into_owned(),
                ));
            }
        }
    }

    Ok(found_files)
}

fn package_contains_file(package: &str, file: &str) -> Result<bool> {
    let list_packages_args = vec!["-ql", package];

    let output = Command::new(RPM_EXECUTABLE)
        .args(list_packages_args.clone())
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
        .context(format!("Cannot get package content of {package}")));
    }
    let stdout = String::from_utf8(output.stdout)?;
    for line in stdout.lines() {
        if line.contains(file) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn package_name_of_lib(lib: &String) -> Result<String> {
    let list_packages_args = vec!["--query", "--all", "--queryformat", "%{NAME}\\n"];

    let output = Command::new(RPM_EXECUTABLE)
        .args(list_packages_args.clone())
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
        .context(format!("Cannot get package name for library {lib}")));
    }
    let mut packages: Vec<_> = vec![];
    let stdout = String::from_utf8(output.stdout)?;
    for package in stdout.lines() {
        if package_contains_file(package, lib)? {
            packages.push(package.to_string());
        }
    }
    if packages.len() > 1 {
        return Err(anyhow::anyhow!(
            "Cannot find unique package containing the library '{lib}': {}",
            packages.join(", ")
        ));
    }
    if let Some(package) = packages.first() {
        return Ok(package.clone());
    }
    Err(anyhow::anyhow!(
        "Cannot find any package containing the library '{lib}'"
    ))
}

fn query_package_info(package: &str, info: &str) -> Result<Option<String>> {
    let query = format!("%{{{info}}}");
    let list_packages_args = vec!["-q", package, "--queryformat", query.as_str()];

    trace!(
        "Query package info: {} {}",
        RPM_EXECUTABLE,
        list_packages_args.join(" ")
    );
    let output = Command::new(RPM_EXECUTABLE)
        .args(list_packages_args.clone())
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
        .context(format!("Cannot get query {info} for package {package}")));
    }
    let stdout = String::from_utf8(output.stdout)?;
    if stdout.is_empty() {
        return Ok(None);
    }
    Ok(Some(stdout))
}

pub fn package_info(package: &String, lib_info: &FileInfo, overrides: &[settings::Override]) -> Result<LicenseInfo> {
    let override_info = settings::Override::find_override(package, overrides);

    let license = if override_info.is_some_and(|x| x.license_id.is_some()) {
        override_info.unwrap().license_id.clone()
    } else {
        query_package_info(package, RPM_QUERY_LICENSE)?
    };
    let version = query_package_info(package, RPM_QUERY_VERSION)?;
    let url = query_package_info(package, RPM_QUERY_URL)?;

    let license = if let Some(license) = license {
        license
    } else {
        return Err(anyhow::anyhow!(
            "Missing license identifier for RPM package {}",
            package
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
        package_collect_files(package, license_file_patterns)?
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
            .context(format!("Cannot parse license expression for package {package}"))
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
                if let Some(expr) = expr {
                    if let Some(license) = &expr.req.license.id() {
                        collection.push(*license);
                    }
                }
            });
        collection
    } else {
        vec![]
    };

    // Detect license ids of license texts:
    let license_texts =
        LicenseDetector::instance().detect_licenses(package, license_ids.as_slice(), license_files.as_slice())?;

    Ok(LicenseInfo {
        lib_info: lib_info.clone(),
        package_name: package.clone(),
        license,
        license_expression,
        license_texts,
        version,
        url,
    })
}
