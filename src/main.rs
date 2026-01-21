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
use clap::{Args, Parser, Subcommand};
use flexi_logger::Logger;
use log::*;
use sancus_lib::{
    file_info,
    file_info::FileInfo,
    license_detector::LicenseDetector,
    license_info::LicenseInfo,
    rpm_info, settings,
    third_party_licenses::{self, ThirdPartyLicenses},
    vcpkg_spdx::{get_license_info, parse_spdx_files},
};
use std::{fs, ops::Deref, panic, path::PathBuf};

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parse a project and create third party information for it
    Create(ExtractFromProductArgs),
    /// Export third party license files to a directory
    Export(ExportFromJsonArgs),
}

#[derive(Args, Debug)]
struct ExtractFromProductArgs {
    /// Package name
    #[arg(long)]
    package_name: String,
    /// Path to the project root directory
    #[arg(long)]
    project_path: PathBuf,
    /// Path to the package directory
    #[arg(long)]
    package_path: PathBuf,
    /// Path to the result directory
    #[arg(long)]
    result_path: PathBuf,
    /// Export human readable representation
    #[arg(long)]
    export_path: Option<PathBuf>,
    /// Path to crates licenses
    #[arg(long)]
    additional_third_party_licenses: Vec<PathBuf>,
}

#[derive(Args, Debug)]
struct ExportFromJsonArgs {
    /// Export human readable representation
    #[arg(requires = "third_party_licenses")]
    export_path: PathBuf,
    /// Path to third party license files
    third_party_licenses: Vec<PathBuf>,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

const SYSTEM_THIRD_PARTY_LICENSES_FILE: &str = "system_third_party_licenses.json";
const VCPKG_THIRD_PARTY_LICENSES_FILE: &str = "vcpkg_third_party_licenses.json";

fn find_package_system_libs(package_libs: &Vec<FileInfo>, vcpkg_libs: &[FileInfo]) -> Vec<FileInfo> {
    let mut system_libs = vec![];
    for lib in package_libs {
        if !vcpkg_libs.iter().any(|l| l.name == lib.name) {
            system_libs.push(lib.clone());
        }
    }
    system_libs
}

fn system_libs_info(system_libs: &Vec<FileInfo>, overrides: &[settings::Override]) -> Result<Vec<LicenseInfo>> {
    let mut system_lib_info: Vec<LicenseInfo> = vec![];
    for lib_info in system_libs {
        trace!("Query system info of '{}'", lib_info.name);
        let package = rpm_info::package_of_lib(&lib_info.name)?;
        if !system_lib_info.iter().any(|info| info.package_name == package.name()) {
            let info = package.license_info(lib_info.clone(), overrides)?;
            system_lib_info.push(info);
        }
    }
    Ok(system_lib_info)
}

fn run(args: &Cli) -> Result<()> {
    match &args.command {
        Commands::Create(args) => {
            export_from_product(args)?;
        }
        Commands::Export(args) => {
            export_from_json(args)?;
        }
    }
    Ok(())
}

fn export_from_product(args: &ExtractFromProductArgs) -> Result<()> {
    let mut vcpkg_find_ignore_list = vec!["debug".to_owned()];

    #[cfg(target_os = "linux")]
    {
        vcpkg_find_ignore_list.push("x64-linux".to_owned()); // Only used to provide build tools
    }

    debug!("project = {:?}, package = {:?}", args.project_path, args.package_path);

    let settings_file = args.project_path.join(settings::Settings::default_settings_file());
    let settings = if settings_file.is_file() {
        settings::Settings::load(&args.project_path.join(settings::Settings::default_settings_file()))?
    } else {
        settings::Settings::default()
    };

    for replace in &settings.overrides {
        trace!(
            "Found override for {} in settings. Comment: {}",
            replace.package, replace.comment
        )
    }

    for vcpkg_ignore in &settings.vcpkg_ignores {
        let add_ignore = if let Some(os) = &vcpkg_ignore.os {
            os == std::env::consts::OS
        } else {
            true
        };
        if add_ignore {
            info!(
                "Add vcpkg directory {} to ignore list. Comment: {}",
                vcpkg_ignore.directory, vcpkg_ignore.comment
            );
            vcpkg_find_ignore_list.push(vcpkg_ignore.directory.clone());
        }
    }

    let mut package_find_ignore_list = vec!["debug".to_owned()];
    for lib_ignore in &settings.lib_ignores {
        info!(
            "Add library {} to ignore list. Comment: {}",
            lib_ignore.lib, lib_ignore.comment
        );
        package_find_ignore_list.push(lib_ignore.lib.clone());
    }

    // Find vcpkg installation
    let mut vcpkg_installation: Option<PathBuf> = None;
    for entry in fs::read_dir(args.project_path.clone())? {
        let entry = entry?;
        let path = entry.path().clone();
        let name = entry.file_name().to_string_lossy().into_owned();

        trace!("Project files: path={path:?} name={name:?}");

        if path.is_dir() && name == "vcpkg_installed" {
            info!("FOUND vcpkg installation at: {path:?}");
            vcpkg_installation = Some(path);
        }
    }
    let vcpkg_installation = if let Some(vcpkg_installation) = vcpkg_installation {
        vcpkg_installation
    } else {
        return Err(anyhow::anyhow!(
            "Cannot find vcpkg installation in {:?}",
            args.project_path
        ));
    };
    // Collect libraries in the vcpkg installation
    let vcpkg_libs =
        file_info::find_files_recurse(&vcpkg_installation.clone(), ".so", vcpkg_find_ignore_list.as_slice()).context(
            format!("Cannot find '.so' files in '{}'", vcpkg_installation.to_string_lossy()),
        )?;
    let package_libs =
        file_info::find_files_recurse(&args.package_path.clone(), ".so", package_find_ignore_list.as_slice()).context(
            format!("Cannot find '.so' files in '{}'", args.package_path.to_string_lossy()),
        )?;

    // Find all vcpkg.spdx.json files
    let vcpkg_spdx_files = file_info::find_files_recurse(
        &vcpkg_installation,
        "vcpkg.spdx.json",
        vcpkg_find_ignore_list.as_slice(),
    )?;

    parse_spdx_files(&vcpkg_spdx_files)?;

    let vcpkg_licenses = get_license_info(&vcpkg_spdx_files, settings.overrides.as_slice())?;

    let vcpkg_third_party_licenses =
        third_party_licenses::ThirdPartyLicenses::new(format!("{}-vcpkg", args.package_name).as_str(), &vcpkg_licenses);
    vcpkg_third_party_licenses.save(&args.result_path.join(VCPKG_THIRD_PARTY_LICENSES_FILE))?;
    vcpkg_third_party_licenses.print();

    // Find libs that are either from vcpkg or from the system:
    let system_libs = find_package_system_libs(&package_libs, &vcpkg_libs);

    let system_licenses = system_libs_info(&system_libs, settings.overrides.as_slice())?;

    let system_third_party_licenses = third_party_licenses::ThirdPartyLicenses::new(
        format!("{}-system", args.package_name).as_str(),
        &system_licenses,
    );
    system_third_party_licenses.save(&args.result_path.join(SYSTEM_THIRD_PARTY_LICENSES_FILE))?;
    system_third_party_licenses.print();

    for crates_licenses_file in &args.additional_third_party_licenses {
        let crates_third_party_licenses_orig = ThirdPartyLicenses::load(crates_licenses_file)?;
        let mut crates_third_party_licenses = crates_third_party_licenses_orig.clone();

        crates_third_party_licenses.apply_overrides(settings.overrides.as_slice())?;

        if crates_third_party_licenses != crates_third_party_licenses_orig {
            crates_third_party_licenses.save(crates_licenses_file)?;
        }

        crates_third_party_licenses.print();
        if let Some(export_path) = &args.export_path {
            crates_third_party_licenses.export(export_path)?;
        }
    }

    if let Some(export_path) = &args.export_path {
        vcpkg_third_party_licenses.export(export_path)?;
        system_third_party_licenses.export(export_path)?;
    }

    Ok(())
}

fn export_from_json(args: &ExportFromJsonArgs) -> Result<()> {
    for crates_licenses_file in &args.third_party_licenses {
        let third_party_licenses = ThirdPartyLicenses::load(crates_licenses_file)?;

        third_party_licenses.print();
        third_party_licenses.export(&args.export_path)?;
    }
    Ok(())
}

fn logging_init() {
    Logger::try_with_env_or_str("trace")
        .expect("Cannot init logging")
        .start()
        .expect("Cannot start logging");

    panic::set_hook(Box::new(|panic_info| {
        let (filename, line, column) = panic_info
            .location()
            .map(|loc| (loc.file(), loc.line(), loc.column()))
            .unwrap_or(("<unknown>", 0, 0));
        let cause = panic_info.payload().downcast_ref::<String>().map(String::deref);
        let cause = cause.unwrap_or_else(|| {
            panic_info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .unwrap_or("<cause unknown>")
        });

        error!(
            "Thread '{}' panicked at {}:{}:{}: {}",
            std::thread::current().name().unwrap_or("<unknown>"),
            filename,
            line,
            column,
            cause
        );
    }));
}

fn main() -> std::process::ExitCode {
    let args = Cli::parse();

    logging_init();
    LicenseDetector::init();

    if let Err(error) = run(&args) {
        error!("{error:?}");
        return std::process::ExitCode::FAILURE;
    }
    std::process::ExitCode::SUCCESS
}
