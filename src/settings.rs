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
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LicenseFileOverride {
    pub id: Option<String>,
    pub file: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(default, rename_all = "snake_case")]
pub struct Override {
    pub package: String,
    pub comment: String,
    pub license_id: Option<String>,
    pub overwrite_all_license_ids: bool,
    pub license_files: Vec<LicenseFileOverride>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct VcpkgIgnore {
    pub directory: String,
    pub comment: String,
    pub os: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct LibIgnore {
    pub lib: String,
    pub comment: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub struct Settings {
    #[serde(default = "default_overrides")]
    pub overrides: Vec<Override>,
    #[serde(default = "default_use_vcpkg_default_ignores")]
    pub use_vcpkg_default_ignores: bool,
    #[serde(default = "default_vcpkg_ignores")]
    pub vcpkg_ignores: Vec<VcpkgIgnore>,
    #[serde(default = "default_lib_ignores")]
    pub lib_ignores: Vec<LibIgnore>,
}

fn default_overrides() -> Vec<Override> {
    vec![]
}

fn default_use_vcpkg_default_ignores() -> bool {
    true
}

fn default_vcpkg_ignores() -> Vec<VcpkgIgnore> {
    vec![]
}

fn default_lib_ignores() -> Vec<LibIgnore> {
    vec![]
}

impl Override {
    pub fn find_override<'a>(package: &str, overrides: &'a [Self]) -> Option<&'a Self> {
        overrides.iter().find(|&x| x.package == package)
    }
}

impl Settings {
    pub fn default_settings_file() -> std::path::PathBuf {
        std::path::Path::new("mithra.yaml").to_path_buf()
    }

    pub fn load(file: &std::path::Path) -> Result<Self> {
        let str = std::fs::read_to_string(file)
            .with_context(|| format!("Cannot read settings file {}", file.to_str().unwrap_or("empty")).clone())?;
        let mut settings = serde_yaml::from_str::<Self>(str.as_str())
            .with_context(|| format!("Cannot parse settings {}", file.to_str().unwrap_or("empty")).clone())?;

        // Map relative license file overwrites to settings file:
        let settings_path = file
            .parent()
            .unwrap_or_else(|| panic!("Cannot get directory of settings file {}", file.to_string_lossy()));
        settings.overrides.iter_mut().for_each(|or| {
            or.license_files.iter_mut().for_each(|lf| {
                let file_path = Path::new(&lf.file);
                if file_path.is_relative() {
                    lf.file = settings_path.join(file_path).to_string_lossy().into_owned()
                }
            });
        });
        Ok(settings)
    }
}
