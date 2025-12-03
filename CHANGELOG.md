# Changelog
All notable changes to this project will be documented in this file. See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.

- - -

## [0.1.7](https://github.com/x-software-com/sancus/compare/v0.1.6...v0.1.7) - 2025-12-03

### Other

- update dependencies
- *(deps)* bump crate-ci/typos from 1.39.2 to 1.40.0
- add feature to scan for licenses and create a JSON-SPDX file
- default features of all dependencies disabled

## [0.1.6](https://github.com/x-software-com/sancus/compare/v0.1.5...v0.1.6) - 2025-11-25

### Other

- use fully qualified versions in Cargo.toml

## [0.1.5](https://github.com/x-software-com/sancus/compare/v0.1.4...v0.1.5) - 2025-11-25

### Other

- fix rust version
- *(deps)* upgrade dependencies
- *(deps)* bump clap from 4.5.51 to 4.5.53
- *(deps)* bump actions/checkout from 5 to 6
- *(deps)* bump crate-ci/typos from 1.39.0 to 1.39.2
- *(deps)* bump serde_yaml_bw from 2.4.1 to 2.5.1
- *(deps)* bump crate-ci/typos from 1.38.1 to 1.39.0
- add cargo machete to commit hooks
- upgrade dependencies
- *(deps)* bump clap from 4.5.49 to 4.5.50
- improve nixos develop experience
- *(deps)* bump regex from 1.11.3 to 1.12.2
- *(deps)* bump clap from 4.5.48 to 4.5.49
- use cocogitto-action v4 instead of main
- *(deps)* bump flexi_logger from 0.31.6 to 0.31.7
- *(deps)* bump crate-ci/typos from 1.37.2 to 1.38.1

## [0.1.4](https://github.com/x-software-com/sancus/compare/v0.1.3...v0.1.4) - 2025-10-08

### Other

- add shell.nix
- cargo update
- *(deps)* bump clap from 4.5.40 to 4.5.48
- *(deps)* bump anyhow from 1.0.98 to 1.0.100
- *(deps)* bump crate-ci/typos from 1.34.0 to 1.37.2
- *(deps)* bump serde_yaml_bw from 2.0.1 to 2.4.1
- *(deps)* bump serde_json from 1.0.140 to 1.0.145
- *(deps)* bump spdx from 0.10.8 to 0.12.0
- *(deps)* bump actions/checkout from 4 to 5

## [0.1.3](https://github.com/x-software-com/sancus/compare/v0.1.2...v0.1.3) - 2025-07-07

### Other

- add missing end quote
- remove no longer needed check
- *(deps)* bump crate-ci/typos from 1.33.1 to 1.34.0

## [0.1.2](https://github.com/x-software-com/sancus/compare/v0.1.1...v0.1.2) - 2025-07-02

### Other

- fix some clippy warnings
- add default recipe in justfile
- *(deps)* bump clap from 4.5.31 to 4.5.40
- *(deps)* bump crate-ci/typos from 1.30.1 to 1.33.1
- *(deps)* bump anyhow from 1.0.96 to 1.0.98
- *(deps)* bump serde from 1.0.218 to 1.0.219
- *(deps)* bump log from 0.4.26 to 0.4.27
- *(deps)* bump serde_json from 1.0.139 to 1.0.140
- *(deps)* bump crate-ci/typos from 1.29.9 to 1.30.1

## [0.1.1](https://github.com/x-software-com/sancus/compare/v0.1.0...v0.1.1) - 2025-02-27

### Other

- cargo update
- update rust edition to 2024
- apply cargo fmt changes
- *(deps)* bump clap from 4.5.29 to 4.5.30
- *(deps)* bump serde from 1.0.217 to 1.0.218
- *(deps)* bump serde_json from 1.0.138 to 1.0.139
- *(deps)* bump anyhow from 1.0.95 to 1.0.96
- *(deps)* bump log from 0.4.25 to 0.4.26
- *(deps)* bump crate-ci/typos from 1.29.7 to 1.29.9
- *(deps)* bump clap from 4.5.28 to 4.5.29
- *(deps)* bump crate-ci/typos from 1.29.5 to 1.29.7
- *(deps)* bump spdx from 0.10.7 to 0.10.8
- *(deps)* bump clap from 4.5.27 to 4.5.28
- *(deps)* bump log from 0.4.22 to 0.4.25
- *(deps)* bump serde_json from 1.0.133 to 1.0.138
- *(deps)* bump serde from 1.0.215 to 1.0.217
- *(deps)* bump clap from 4.5.21 to 4.5.27
- *(deps)* bump anyhow from 1.0.93 to 1.0.95
- *(deps)* bump flexi_logger from 0.29.6 to 0.29.8
- *(deps)* bump crate-ci/typos from 1.28.2 to 1.29.5
- *(deps)* bump crate-ci/typos from 1.28.1 to 1.28.2
- replaced once_cell dependency
- *(deps)* bump crate-ci/typos from 1.27.3 to 1.28.1
- improve README.md
- add machete and typos to justfile
- fix typos

## [v0.1.0](https://github.com/x-software-com/sancus/compare/96aa8bf492ffb107c1f56fa615c41ccd193e9d65..v0.1.0) - 2024-02-19
#### Bug Fixes
- improve handling of invalid filenames - ([23ef721](https://github.com/x-software-com/sancus/commit/23ef7219d267e187d3e5f46912e8e5fe2073d3b7)) - marcbull
- fix problem with unusual license IDs - ([65f2296](https://github.com/x-software-com/sancus/commit/65f22965517a1f2b23035f7e0ecf7ff992f19b48)) - marcbull
#### Build system
- add setup.py - ([7630030](https://github.com/x-software-com/sancus/commit/7630030fcf02e955507a7ca284df3e7b79e07843)) - marcbull
#### Continuous Integration
- build on ubuntu-latest - ([1e95771](https://github.com/x-software-com/sancus/commit/1e9577112c2d8ec2d1b7cfbdbd0dcc327d18a536)) - marcbull
- update check workflow for open source release - ([baa1e30](https://github.com/x-software-com/sancus/commit/baa1e308fe08b9c8382ecfd37143f461563b21f8)) - marcbull
- use rust cache - ([3880717](https://github.com/x-software-com/sancus/commit/3880717c92c00258e27c3a6ddd95edc4ad0a1723)) - marcbull
- use current almalinux docker image - ([ca6bc04](https://github.com/x-software-com/sancus/commit/ca6bc047f7a4ca0385f2f4c15f1fc725ab9580aa)) - marcbull
- fix check.yml - ([4fbe7a5](https://github.com/x-software-com/sancus/commit/4fbe7a5404cb87a0ee8afd281c13d5f4ca99c16a)) - marcbull
- add github workflows - ([21941f8](https://github.com/x-software-com/sancus/commit/21941f80002ca86a375b513726182ca04b22c379)) - marcbull
#### Documentation
- add README.md and CONTRIBUTING.md - ([6db31f6](https://github.com/x-software-com/sancus/commit/6db31f67b306ec0d3faf4ade35559a2f9ff26a4e)) - marcbull
#### Miscellaneous Chores
- **(deps)** bump clap from 4.4.8 to 4.5.0 (#33) - ([564f6d6](https://github.com/x-software-com/sancus/commit/564f6d66ce474078d52229d9a23927302893b89a)) - dependabot[bot]
- **(deps)** bump crate-ci/typos from 1.16.23 to 1.18.2 (#32) - ([9ee68a1](https://github.com/x-software-com/sancus/commit/9ee68a1ee15b2fa9dca71917e8e286a57e9825de)) - dependabot[bot]
- **(deps)** bump serde_yaml from 0.9.27 to 0.9.31 (#30) - ([c04a917](https://github.com/x-software-com/sancus/commit/c04a917fcee42c6728476a6061d93f73b7b8bf6c)) - dependabot[bot]
- **(deps)** bump serde from 1.0.192 to 1.0.196 (#29) - ([b181f65](https://github.com/x-software-com/sancus/commit/b181f65713374e50270ac7ff5903c6a8d792313a)) - dependabot[bot]
- **(deps)** bump anyhow from 1.0.75 to 1.0.79 (#21) - ([10dde39](https://github.com/x-software-com/sancus/commit/10dde39f1c0d65bbaa3e6e73e3ff767c0e934d71)) - dependabot[bot]
- **(deps)** bump once_cell from 1.18.0 to 1.19.0 (#13) - ([a55ac52](https://github.com/x-software-com/sancus/commit/a55ac5263b7892168a0d692f9543fcdaf2b77065)) - dependabot[bot]
- **(deps)** bump clap from 4.3.19 to 4.4.8 (#8) - ([258fdbf](https://github.com/x-software-com/sancus/commit/258fdbf6349c06ca9d8cf8fdc7a56327cad9603b)) - dependabot[bot]
- **(deps)** bump serde_json from 1.0.103 to 1.0.108 (#7) - ([8050531](https://github.com/x-software-com/sancus/commit/8050531935e6c6923a0db6a5404f9d75d614ffca)) - dependabot[bot]
- **(deps)** bump anyhow from 1.0.72 to 1.0.75 - ([c3da468](https://github.com/x-software-com/sancus/commit/c3da468e41dbbf82d780dffc6722e0e446c2eb16)) - dependabot[bot]
- **(deps)** bump serde_yaml from 0.9.25 to 0.9.27 - ([43d54bf](https://github.com/x-software-com/sancus/commit/43d54bfcd7109c033817fe6c045882ff2730e3c2)) - dependabot[bot]
- **(deps)** bump spdx-rs from 0.5.3 to 0.5.5 - ([6535b68](https://github.com/x-software-com/sancus/commit/6535b682cabeda422ecbf0a6981186d33206433d)) - dependabot[bot]
- **(deps)** bump regex from 1.9.1 to 1.10.2 - ([b1bda14](https://github.com/x-software-com/sancus/commit/b1bda14a474737ac5a7e130c0e01faa24502380b)) - dependabot[bot]
- **(deps)** bump log from 0.4.19 to 0.4.20 - ([9832164](https://github.com/x-software-com/sancus/commit/9832164c31f3b2cf99e90c97b182b4685c1e9f9f)) - dependabot[bot]
- **(deps)** bump crate-ci/typos from 1.16.22 to 1.16.23 - ([c8a2da5](https://github.com/x-software-com/sancus/commit/c8a2da57b19918f0d23404501cc63ce91f316f93)) - dependabot[bot]
- add --overwrite argument to cog install-hook - ([62695bb](https://github.com/x-software-com/sancus/commit/62695bbd0f03922216fd3181eca5f66fe716dc87)) - marcbull
- use build:3 image - ([9571f41](https://github.com/x-software-com/sancus/commit/9571f4173ecd4097f0a2807e4e3d01621e993eb9)) - marcbull
- updates for open source release - ([d300cc1](https://github.com/x-software-com/sancus/commit/d300cc1f9191f5fecffd30d80d88cf33637d40e8)) - marcbull
- add licenses - ([d34b5ce](https://github.com/x-software-com/sancus/commit/d34b5ceed60cb9204d3d8929e07d73c77a318664)) - marcbull
- add setting to overwrite all license IDs of the license files - ([080d13f](https://github.com/x-software-com/sancus/commit/080d13fe4a46f5d76d64d8ba0b4954044d26e3d1)) - marcbull
- update setup.py - ([e78c510](https://github.com/x-software-com/sancus/commit/e78c5105ae9d7126dbbcdc58ce4ed66898dbdd6b)) - marcbull
- check for typos in commit message - ([7e246eb](https://github.com/x-software-com/sancus/commit/7e246ebef27ac10dfc47e7af1f97537334d32c0b)) - marcbull
- add rustfmt.toml - ([b22ea75](https://github.com/x-software-com/sancus/commit/b22ea759800b4d7e4cb076a94b7241787c170854)) - marcbull
- install cargo-edit for set-version and update cog.toml - ([772e18b](https://github.com/x-software-com/sancus/commit/772e18bf21d38fd84bdd5e176676fedce86b29eb)) - marcbull
- update cog.toml - ([29f0ab7](https://github.com/x-software-com/sancus/commit/29f0ab7eb567c0f9f9987880a739bb761d9480b5)) - marcbull
- fix typos - ([dbf2f92](https://github.com/x-software-com/sancus/commit/dbf2f92ff39b2526590392e3681c27f8ec5dc2c2)) - marcbull
- apply auto-format to Cargo.toml - ([aeef74e](https://github.com/x-software-com/sancus/commit/aeef74e34ed38e088e88680e46d8969e5711ee83)) - marcbull
- update Cargo.toml - ([034cb2f](https://github.com/x-software-com/sancus/commit/034cb2ff426e786311be2090a71b92095836dd4a)) - marcbull
- apply auto-format for cog.toml - ([fa69f2a](https://github.com/x-software-com/sancus/commit/fa69f2ad86db8093ce83f3b07aa7979ca422d08e)) - marcbull
- enable ignore_merge_commits and update changelog data cog.toml - ([72f5e9e](https://github.com/x-software-com/sancus/commit/72f5e9e2f425870683a06e80836532cde726747a)) - marcbull

- - -
