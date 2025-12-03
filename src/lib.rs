// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// SPDX-FileCopyrightText: 2024 X-Software GmbH <opensource@x-software.com>

pub mod file_info;
pub mod license_info;
pub mod license_text;
pub mod third_party_licenses;

#[cfg(feature = "scan")]
pub mod license_detector;
#[cfg(feature = "scan")]
pub mod rpm_info;
#[cfg(feature = "scan")]
pub mod settings;
#[cfg(feature = "scan")]
pub mod vcpkg_spdx;
