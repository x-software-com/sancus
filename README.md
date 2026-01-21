[![CI checks](https://github.com/x-software-com/sancus/actions/workflows/check.yml/badge.svg)](https://github.com/x-software-com/sancus/actions/workflows/check.yml)
[![dependency status](https://deps.rs/repo/github/x-software-com/sancus/status.svg)](https://deps.rs/repo/github/x-software-com/sancus)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

# Sancus: Overview

**NOTE:** Sancus currently expects deep integration with the build system, and is difficult to deploy to environments dissimilar to the X-Software environment. We do not have our tooling set up to accept external contributions at this time.

Sancus is a tool designed to generate a SPDX (Software Package Data Exchange) file containing information about third-party licenses used in software packages. The goal is to collect comprehensive license information beyond what is available through existing tools like cargo-bundle-licenses, which focus on identifying third-party libraries used by crates, but this view often falls short because it doesn't take into account linked libraries. Sancus aims to fill this gap by scanning the AppDir identifying the libraries and finding license information using VCPKG and RPM package metadata.

With Sancus, developers can easily identify and document the licenses associated with third-party dependencies in their projects. This makes it easier to manage intellectual property rights, ensure compliance with licensing requirements and maintain transparency in software development.

Currently, Sancus works exclusively with [AppDir](https://github.com/TheAssassin/linuxdeploy/wiki/AppDir-specification) packages consisting of VCPKG libraries and operating system libraries from RPM-based Linux distributions. It collects information from both sources to extract the licenses of all shared libraries that an application directly or indirectly uses within the AppDir. The result is a JSON SPDX file that summarizes the results, providing a clear and concise record of the third-party licenses used in the project.

In addition to creating SPDX files, Sancus can also extract the contents of an existing SPDX file into a directory structure.

While Sancus is currently limited in scope, it demonstrates a useful capability for managing license information in certain software ecosystems. As the tool evolves, support for other package formats and systems may be added.

If you've never used Sancus before, or if you're trying to figure out how to use it, check out our [Getting Started](#getting-started) section.

# Table of Contents

- [Sancus: Overview](#sancus-overview)
- [Table of Contents](#table-of-contents)
- [Getting Started](#getting-started)
  - [Quick Start](#quick-start)
  - [Installing Linux Developer Tools](#installing-linux-developer-tools)
- [Contributing](#contributing)
- [License](#license)

# Getting Started

## Quick Start

### Integration into a rust project

Prerequisites:
- [Git][getting-started:git]
- [Rust][getting-started:rust] >= 1.92

Execute the following command:

```
cargo add sancus
```

### Development: Linux

Prerequisites:
- [Git][getting-started:git]
- [Rust][getting-started:rust] >= 1.92

First, download and setup the repository.

```sh
$ cargo install --locked just
$ git clone https://github.com/x-software-com/sancus
$ cd sancus
$ just setup
```

To build the Sancus execute:

```sh
$ cargo build
```

## Installing Linux Developer Tools

Across the different distributions of Linux, there are different packages you'll need to install:

- AlmaLinux, Rocky Linux, CentOS and other RedHat-based distributions:

```sh
$ sudo dnf install gcc git
```

- Debian, Ubuntu, popOS, and other Debian-based distributions:

```sh
$ sudo apt-get update
$ sudo apt-get install build-essential git
```

- ArchLinux, Manjaro Linux, EndeavourOS, and other ArchLinux-based distributions:

```sh
$ sudo pacman -Sy git
```

For any other distributions, make sure you're installing at least gcc and g++. If you want to add instructions for your specific distribution, [please open a PR][contributing:submit-pr]!

[getting-started:git]: https://git-scm.com/downloads
[getting-started:rust]: https://rustup.rs/

# Privacy

We believe that privacy is a human right, period.

Sancus does respect your privacy, we collect no data and do not send any telemetry or usage data.

# License

The code in this repository is licensed under either of [Apache-2.0 License](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
