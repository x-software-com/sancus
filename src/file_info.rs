// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
//
// SPDX-License-Identifier: MIT OR Apache-2.0
//
// SPDX-FileCopyrightText: 2024 X-Software GmbH <opensource@x-software.com>

use anyhow::Result;
use std::{
    fs,
    path::{Path, PathBuf},
};
#[derive(Debug, Clone)]
pub struct FileInfo {
    pub name: String,
    pub path: PathBuf,
    pub extension: Option<String>,
}

impl FileInfo {
    pub fn new(name: String, path: &Path) -> Self {
        Self {
            name,
            path: path.to_path_buf(),
            extension: path.extension().map(|e| e.to_string_lossy().into_owned()),
        }
    }
}

pub fn find_files_recurse(path: &PathBuf, filter: &str, ignore_list: &[String]) -> Result<Vec<FileInfo>> {
    let mut libs: Vec<_> = vec![];
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path().clone();
        let name = entry.file_name().to_string_lossy().into_owned();

        if path.is_dir() && !ignore_list.contains(&name) {
            let sub_libs = find_files_recurse(&path, filter, ignore_list)?;
            for lib in sub_libs {
                libs.push(lib);
            }
        } else if path.is_file() && name.contains(filter) && !ignore_list.contains(&name) {
            libs.push(FileInfo::new(name, path.as_path()));
        }
    }

    Ok(libs)
}
