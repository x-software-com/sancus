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
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::license_info::LicenseInfo;
use crate::settings;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct License {
    pub license: String,
    pub text: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ThirdPartyLibrary {
    pub package_name: String,
    pub package_version: String,
    pub license: String,
    pub licenses: Vec<License>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ThirdPartyLicenses {
    pub root_name: String,
    pub third_party_libraries: Vec<ThirdPartyLibrary>,
}

impl ThirdPartyLicenses {
    pub fn load(file: &std::path::Path) -> Result<Self> {
        let str = std::fs::read_to_string(file).with_context(|| {
            format!(
                "Cannot read third party licenses from file {}",
                file.to_str().unwrap_or("empty")
            )
        })?;
        serde_json::from_str::<Self>(str.as_str()).with_context(|| {
            format!(
                "Cannot parse third party licenses from file {}",
                file.to_str().unwrap_or("empty")
            )
        })
    }

    pub fn save(&self, file_path: &Path) -> Result<()> {
        // Create all parent directories if the don't exist:
        let parent_dir = file_path.parent().unwrap_or_else(|| {
            panic!(
                "Cannot get parent directory of third party file {}",
                file_path.to_string_lossy().into_owned()
            )
        });
        std::fs::create_dir_all(parent_dir)?;
        // Write json file:
        let str = serde_json::to_string_pretty(self).with_context(|| "Cannot serialize third party licenses")?;
        std::fs::write(file_path, str).with_context(|| {
            format!(
                "Cannot serialize third party licenses to file {}",
                file_path.to_str().unwrap_or("empty")
            )
        })
    }

    pub fn apply_overrides(&mut self, overrides: &[settings::Override]) -> Result<()> {
        for package in &mut self.third_party_libraries {
            if let Some(license_override) = settings::Override::find_override(&package.package_name, overrides) {
                if let Some(license_id) = &license_override.license_id {
                    package.license = license_id.clone();

                    if license_override.overwrite_all_license_ids {
                        for package_license in &mut package.licenses {
                            package_license.license = license_id.clone();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn export(&self, result_dir: &Path) -> Result<()> {
        let base_path = result_dir.join(self.root_name.clone());
        for package in &self.third_party_libraries {
            let pkg_dir = base_path.join(package.package_name.clone());

            std::fs::create_dir_all(pkg_dir.as_path())?;

            let pkg_desc_file = pkg_dir.join("README.txt");
            let pkg_desc = format!(
                "Package: {}\nVersion: {}\nLicense: {}\n",
                package.package_name, package.package_version, package.license
            );
            std::fs::write(pkg_desc_file.as_path(), pkg_desc).with_context(|| {
                format!(
                    "Cannot write package description to file {}",
                    pkg_desc_file.to_string_lossy()
                )
            })?;

            for license_text in &package.licenses {
                // Workaround to fix unusual license ID: "License specified in file ($CARGO_HOME/registry/src/.../LICENSE)"
                let file_name = license_text.license.replace(std::path::is_separator, "-");
                let file_name = file_name.replace('$', "-");
                let license_text_file = pkg_dir.join(format!("{file_name}.txt"));

                std::fs::write(license_text_file.as_path(), license_text.text.clone()).with_context(|| {
                    format!(
                        "Cannot write license text to file {}",
                        license_text_file.to_string_lossy()
                    )
                })?;
            }
        }
        Ok(())
    }

    pub fn new(root_name: &str, license_infos: &[LicenseInfo]) -> Self {
        let mut third_party_libraries = vec![];

        for info in license_infos {
            let mut licenses = vec![];

            for license_text in &info.license_texts {
                licenses.push(License {
                    license: license_text.id.clone(),
                    text: license_text.text.clone(),
                });
            }

            third_party_libraries.push(ThirdPartyLibrary {
                package_name: info.package_name.clone(),
                package_version: info.version.clone().unwrap_or_default(),
                license: info.license.clone(),
                licenses,
            });
        }

        ThirdPartyLicenses {
            root_name: root_name.to_owned(),
            third_party_libraries,
        }
    }

    pub fn print(&self) {
        let mut root_tree = termtree::Tree::new(self.root_name.clone());

        for lib in &self.third_party_libraries {
            let mut pkg_tree = termtree::Tree::new(format!("Package: {}", lib.package_name.clone()));

            pkg_tree.push(termtree::Tree::new(format!("Version: {}", lib.package_version)));
            pkg_tree.push(termtree::Tree::new(format!("License ID: {}", lib.license)));

            let mut license_tree = termtree::Tree::new("Licenses:".to_owned());
            for license in &lib.licenses {
                license_tree.push(termtree::Tree::new(format!("ID: {}", license.license)));
            }
            pkg_tree.push(license_tree);

            root_tree.push(pkg_tree);
        }

        for line in root_tree.to_string().lines() {
            debug!("{line}")
        }
    }
}
