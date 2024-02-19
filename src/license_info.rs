// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// SPDX-FileCopyrightText: 2024 X-Software GmbH <opensource@x-software.com>

use crate::{file_info::FileInfo, license_text::LicenseText};

#[derive(Debug, Clone)]
pub struct LicenseInfo {
    pub lib_info: FileInfo,
    pub package_name: String,
    pub license: String,
    pub license_expression: Option<spdx::Expression>,
    pub license_texts: Vec<LicenseText>,
    pub version: Option<String>,
    pub url: Option<String>,
}
