[![CI checks](https://github.com/x-software-com/mithra/actions/workflows/check.yml/badge.svg)](https://github.com/x-software-com/mithra/actions/workflows/check.yml)
[![dependency status](https://deps.rs/repo/github/x-software-com/mithra/status.svg)](https://deps.rs/repo/github/x-software-com/mithra)
[![Conventional Commits](https://img.shields.io/badge/Conventional%20Commits-1.0.0-yellow.svg)](https://conventionalcommits.org)

# Mithra: Overview

**NOTE:** Mithra currently expects deep integration with the build system, and is difficult to deploy to environments dissimilar to the X-Software environment. We do not have our tooling set up to accept external contributions at this time.

Mithra is an open-source tool that extracts third-party license information from a deployment-ready application.

Currently, Mithra solely operates on [AppDir](https://github.com/TheAssassin/linuxdeploy/wiki/AppDir-specification) packages consisting of VCPKG libraries and operating system libraries. It collects information from both sources to extract licenses of all shared libraries that an application uses directly or indirectly within the AppDir. The result is a JSON file containing all license types and the texts.

The obtained third-party license information can be exported to directory structure in the AppDir or the 

If you've never used Mithra before, or if you're trying to figure out how to use it, check out our [Getting Started](#getting-started) section.

# Table of Contents

- [Mithra: Overview](#mithra-overview)
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
- [Rust][getting-started:rust] >= 1.70.0

Add the following line to your `Cargo.toml`.

```
mithra = { git = "https://github.com/x-software-com/mithra" }
```

### Development: Linux

Prerequisites:
- [Git][getting-started:git]
- [Rust][getting-started:rust] >= 1.70.0
- [Python][getting-started:python]

First, download and setup the repository.

```sh
$ cargo install --locked just
$ git clone https://github.com/x-software-com/mithra
$ cd mithra
$ just setup
```

To build the Mithra execute:

```sh
$ cargo build
```

## Installing Linux Developer Tools

Across the different distributions of Linux, there are different packages you'll need to install:

- AlmaLinux, Rocky Linux, CentOS and other RedHat-based distributions:

```sh
$ sudo dnf install gcc python39 git
```

- Debian, Ubuntu, popOS, and other Debian-based distributions:

```sh
$ sudo apt-get update
$ sudo apt-get install build-essential git python3
```

- ArchLinux, Manjaro Linux, EndeavourOS, and other ArchLinux-based distributions:

```sh
$ sudo pacman -Sy python git
```

For any other distributions, make sure you're installing at least gcc and g++. If you want to add instructions for your specific distribution, [please open a PR][contributing:submit-pr]!

[getting-started:git]: https://git-scm.com/downloads
[getting-started:rust]: https://rustup.rs/
[getting-started:python]: https://www.python.org/downloads/

# Privacy

We believe that privacy is a human right, period.

Mithra does respect your privacy, we collect no data and do not send any telemetry or usage data.

# License

The code in this repository is licensed under either of [APACHE-2.0 License](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
