[![License: GPL v3](https://img.shields.io/badge/LICENSE-GPL_v3-blue?logo=gnu)](https://www.gnu.org/licenses/gpl-3.0)
[![License: AGPL v3](https://img.shields.io/badge/LICENSE-AGPL_v3-blue?logo=gnu)](https://www.gnu.org/licenses/agpl-3.0)

[![Change Log](https://img.shields.io/badge/CHANGELOG-0.1.0-limegreen?logo=markdown)](CHANGELOG.md)

> ## Disclaimer
> *All trademarks, third-party assets/logos, and brand names used in this repository/project are the property of their respective owners. 
> This project is an independent educational resource and is not sanctioned, sponsored, or managed by any third-party trademark holders.*


> ### [!NOTE]
> **Cargo package**, which represent a concrete project. It can contain:
> + Zero or more `binary crates` (can contain *unit test*)
> + Zero or one `library crate` (can contain *unit test*)
> + Zero or more `bench` | `integration test`
> #### [!IMPORTANT]
> A package must contain at least one crate; either a `library crate`, a `binary crate`, or both

## Overview cargo project structure
```bash
workspace ---> package (concrete project) ---> crate ---> module
```

## Basic Cargo command(s)
### workspace basic cargo command
```bash
workspace > cargo test [--release] [-p <package>] [--all] [--] [--no-capture]
workspace > cargo test [--release] [-p <package>] [--test <integration_test_name>]
workspace > cargo test [--release] [-p <package>] [--bin <bin_crate_name>]
workspace > cargo test [--release] [-p <package>] [--lib]
workspace > cargo build [--release] [-p <package>] [--bin <bin_crate_name>]
workspace > cargo build [--release] [-p <package>] [--lib]
workspace > cargo run [--release] [-p <package>] [--bin <bin_crate_name>]
```
### project basic cargo command
```bash
package > cargo test [--release] [--all] [--] [--no-capture] 
package > cargo test [--release] [--test <integration_test_name>]
package > cargo test [--release] [--bin <bin_crate_name>]
package > cargo test [--release] [--lib]
package > cargo build [--release] [--bin <bin_crate_name>]
package > cargo build [--release] [--lib]
package > cargo run [--release] [--bin <bin_crate_name>]
```