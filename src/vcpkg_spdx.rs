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
use spdx_rs::models::PackageInformation;
use std::{fs, path::Path};

use crate::file_info::FileInfo;
use crate::license_detector::{LicenseDetector, LicenseFile};
use crate::license_info::LicenseInfo;
use crate::settings;

const SPDX_ID_PORT: &str = "SPDXRef-port";
const SPDX_ID_BINARY: &str = "SPDXRef-binary";
const SPDX_ID_RESOURCE_1: &str = "SPDXRef-resource-1";

fn parse_spdx_file(file: &Path) -> Result<spdx_rs::models::SPDX> {
    let spdx_string = fs::read_to_string(file)?;
    let clean = spdx_string
        .lines()
        .filter(|x| !x.trim_start().starts_with("\"$schema\":"))
        .collect::<Vec<_>>();
    let spdx_string_clean = clean.join("\n");

    serde_json::from_str(spdx_string_clean.as_ref()).context(format!("Failed to parse SPDX JSON file {:?}", file))
}

fn find_package_by_spdxid<'a>(
    id: &str,
    packages: &'a [spdx_rs::models::PackageInformation],
) -> Option<&'a PackageInformation> {
    packages.iter().find(|&x| x.package_spdx_identifier == id)
}

fn find_package_license(pkg: &PackageInformation) -> Option<String> {
    let mut found_license = None;
    if let Some(license) = pkg.declared_license.as_ref() {
        if license.to_string() != "NOASSERTION" {
            found_license = Some(license.to_string());
        }
    }
    if let Some(license) = pkg.concluded_license.as_ref() {
        if found_license.is_none() && license.to_string() != "NOASSERTION" {
            found_license = Some(license.to_string());
        }
    }
    found_license
}

fn find_license(packages: &[spdx_rs::models::PackageInformation]) -> Option<String> {
    if let Some(info) = find_package_by_spdxid(SPDX_ID_BINARY, packages) {
        if let Some(license) = find_package_license(info) {
            return Some(license);
        }
    }
    if let Some(info) = find_package_by_spdxid(SPDX_ID_PORT, packages) {
        if let Some(license) = find_package_license(info) {
            return Some(license);
        }
    }
    None
}

pub fn get_license_info(
    vcpkg_spdx_files: &Vec<FileInfo>,
    overrides: &[settings::Override],
) -> Result<Vec<LicenseInfo>> {
    let mut licenses = vec![];

    for file in vcpkg_spdx_files {
        let directory = file.path.parent().unwrap();
        let spdx_result = parse_spdx_file(file.path.as_path())?;
        let packages = spdx_result.package_information.as_ref();

        let found_license = find_license(packages);

        let url = find_package_by_spdxid(SPDX_ID_RESOURCE_1, packages).map(|pkg| pkg.package_download_location.clone());

        if let Some(pkg) = find_package_by_spdxid(SPDX_ID_PORT, packages) {
            let override_info = settings::Override::find_override(&pkg.package_name, overrides);

            let license = if override_info.is_some_and(|x| x.license_id.is_some()) {
                override_info.unwrap().license_id.clone()
            } else {
                found_license
            };

            // Find license file for package:
            let copyright_file = directory.join("copyright");
            let license_files = if override_info.is_some_and(|x| !x.license_files.is_empty()) {
                override_info
                    .unwrap()
                    .license_files
                    .iter()
                    .map(|license_file| LicenseFile {
                        id: license_file.id.clone(),
                        file: license_file.file.clone(),
                    })
                    .collect()
            } else if copyright_file.exists() && copyright_file.is_file() {
                vec![LicenseFile {
                    id: None,
                    file: copyright_file.to_string_lossy().into_owned(),
                }]
            } else {
                vec![]
            };

            let license = match license {
                Some(license) => license,
                None => {
                    return Err(anyhow::anyhow!(
                        "Missing license identifier for VCPKG package {}",
                        pkg.package_name
                    ))
                }
            };

            let license_expression = if !license.is_empty() {
                match spdx::Expression::parse(license.as_str()).context(format!(
                    "Cannot parse license expression for package {}",
                    pkg.package_name
                )) {
                    Ok(expr) => Some(expr),
                    Err(error) => {
                        warn!("{:?}", error);
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
            let license_texts = LicenseDetector::instance().detect_licenses(
                pkg.package_name.as_str(),
                license_ids.as_slice(),
                license_files.as_slice(),
            )?;

            licenses.push(LicenseInfo {
                lib_info: FileInfo::new(
                    file.path
                        .file_name()
                        .map_or_else(|| "".to_owned(), |x| x.to_string_lossy().into_owned()),
                    file.path.as_path(),
                ),
                package_name: pkg.package_name.clone(),
                license,
                license_expression,
                license_texts,
                version: pkg.package_version.clone(),
                url,
            })
        }
    }

    Ok(licenses)
}

pub fn parse_spdx_files(vcpkg_spdx_files: &Vec<FileInfo>) -> Result<()> {
    let mut root_tree = termtree::Tree::new("SPDX info tree".to_owned());

    for file in vcpkg_spdx_files {
        let mut tree = termtree::Tree::new(format!("file: {:?}", file.path));

        let spdx_result = parse_spdx_file(file.path.as_path())?;

        for pkg in spdx_result.package_information {
            let sub_tree = parse_package(pkg);

            tree.push(sub_tree);
        }

        root_tree.push(tree);
    }

    for line in root_tree.to_string().lines() {
        debug!("{line}")
    }

    Ok(())
}

fn parse_package(pkg: spdx_rs::models::PackageInformation) -> termtree::Tree<String> {
    let mut sub_tree = termtree::Tree::new(format!(
        "Package: {} ({})",
        pkg.package_name, pkg.package_spdx_identifier
    ));
    if pkg.package_home_page.is_some() {
        sub_tree.push(termtree::Tree::new(format!(
            "Homepage: {}",
            pkg.package_home_page.unwrap()
        )));
    }

    let mut found_license = None;
    if let Some(license) = pkg.declared_license {
        if license.to_string() != "NOASSERTION" {
            found_license = Some(format!("declared license: {}", license));
        }
    }
    if let Some(license) = pkg.concluded_license {
        if found_license.is_none() && license.to_string() != "NOASSERTION" {
            found_license = Some(format!("concluded license: {}", license));
        }
    }
    if found_license.is_none() {
        found_license = Some("FOUND NO LICENSE!".to_owned());
    }

    sub_tree.push(termtree::Tree::new(found_license.unwrap()));

    sub_tree
}
