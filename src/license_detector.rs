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
use once_cell::sync::OnceCell;
use regex::Regex;
use spdx::LicenseId;
use std::path::Path;
use std::{collections::HashMap, io::Write};

use crate::license_text::LicenseText;

#[derive(Debug, Clone)]
pub struct LicenseFile {
    pub id: Option<String>,
    pub file: String,
}

#[derive(Debug, Clone)]
struct LicenseHash {
    id: Option<String>,
    text: String,
    word_hash: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub struct LicenseDetector {
    templates: Vec<LicenseHash>,
}

static INSTANCE: OnceCell<LicenseDetector> = OnceCell::new();

impl LicenseHash {
    pub fn new(id: Option<String>, text: &str) -> Self {
        let word_hash = Self::generate_hash(text);
        Self {
            id,
            text: text.to_owned(),
            word_hash,
        }
    }

    fn generate_hash(text: &str) -> HashMap<String, u32> {
        let mut word_hash = HashMap::new();
        for word in Regex::new(r"\w+").unwrap().find_iter(text) {
            *word_hash.entry(word.as_str().to_lowercase().clone()).or_insert(0) += 1;
        }
        word_hash
    }
}

impl LicenseDetector {
    pub fn init() {
        let license_detector = Self::build();
        INSTANCE.set(license_detector).unwrap();
    }

    pub fn instance() -> &'static Self {
        INSTANCE
            .get()
            .expect("LicenseDetector is not initialized, please execute LicenseDetector::init()")
    }

    fn build() -> Self {
        let mut templates = vec![];
        for (id, _full_name, _flags) in spdx::identifiers::LICENSES {
            let license = spdx::license_id(id).unwrap();

            templates.push(LicenseHash::new(Some(license.name.to_owned()), license.text()));
        }
        LicenseDetector { templates }
    }

    fn compare(text_hash: &HashMap<String, u32>, template_hash: &HashMap<String, u32>) -> u32 {
        let mut errors = 0;
        let mut text_hash = text_hash.clone();

        for (word, &count) in template_hash {
            let text_count = text_hash.remove(word).unwrap_or(0);
            let diff = ((text_count as i32) - (count as i32)).unsigned_abs();
            errors += diff;
        }

        for (_, count) in text_hash {
            errors += count;
        }

        errors
    }

    pub fn detect_license(&self, package: &str, license_ids: &[LicenseId], text: &str) -> LicenseText {
        let mut text_hash = LicenseHash::new(None, text);
        let mut best_score = None;
        let mut best_template_text = "".to_owned();

        if license_ids.is_empty() {
            for template in &self.templates {
                let total: u32 = template.word_hash.values().sum();
                let errors = Self::compare(&text_hash.word_hash, &template.word_hash);
                let score = (errors as f32) / (total as f32);

                trace!("Score for {}: {}", template.id.as_ref().unwrap(), score);
                if best_score.is_none() || score < best_score.unwrap() {
                    best_score = Some(score);
                    text_hash.id = template.id.clone();
                    best_template_text = template.text.to_owned();
                }
            }
        } else {
            for license_id in license_ids {
                let template = LicenseHash::new(Some(license_id.name.to_owned()), license_id.text());
                let total: u32 = template.word_hash.values().sum();
                let errors = Self::compare(&text_hash.word_hash, &template.word_hash);
                let score = (errors as f32) / (total as f32);

                trace!("Score for {}: {}", template.id.as_ref().unwrap(), score);
                if best_score.is_none() || score < best_score.unwrap() {
                    best_score = Some(score);
                    text_hash.id = template.id.clone();
                    best_template_text = template.text.to_owned();
                }
            }
        }
        let score = best_score.unwrap();

        debug!("Best score was for {}: {}", text_hash.id.as_ref().unwrap(), score);

        if license_ids.len() == 1 {
            let license_id = license_ids.first().unwrap();
            if text_hash.id.as_ref().unwrap() != license_id.name {
                let tmp_dir = Path::new("license_detect").join(package);
                std::fs::create_dir_all(tmp_dir.clone()).unwrap();

                let text_file = tmp_dir.join("best_template_text.txt");
                let mut file = std::fs::File::create(text_file).unwrap();
                file.write_all(best_template_text.as_bytes()).unwrap();
                drop(file);

                let text_file = tmp_dir.join("original_text.txt");
                let mut file = std::fs::File::create(text_file).unwrap();
                file.write_all(text_hash.text.as_bytes()).unwrap();
                drop(file);

                let text_file = tmp_dir.join("spdx_id_text.txt");
                let mut file = std::fs::File::create(text_file).unwrap();
                file.write_all(license_id.text().as_bytes()).unwrap();
                drop(file);
            }
        }

        LicenseText {
            id: text_hash.id.unwrap(),
            text: text.to_owned(),
        }
    }

    pub fn detect_licenses(
        &self,
        package: &str,
        license_ids: &[LicenseId],
        license_files: &[LicenseFile],
    ) -> Result<Vec<LicenseText>> {
        debug!("Detect license ids for license texts of package {package}");
        let mut license_texts = vec![];
        for license_file in license_files {
            let text = std::fs::read_to_string(license_file.file.clone())
                .with_context(|| format!("Cannot read third party license file {}", license_file.file))?;

            if let Some(id) = license_file.id.as_ref() {
                license_texts.push(LicenseText { id: id.clone(), text });
            } else {
                license_texts.push(LicenseDetector::instance().detect_license(package, license_ids, text.as_str()));
            }
        }
        Ok(license_texts)
    }
}
