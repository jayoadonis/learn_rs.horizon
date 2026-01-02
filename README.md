[![License: GPL v3](https://img.shields.io/badge/LICENSE-GPL_v3-blue?logo=gnu)](https://www.gnu.org/licenses/gpl-3.0)
[![License: AGPL v3](https://img.shields.io/badge/LICENSE-AGPL_v3-blue?logo=gnu)](https://www.gnu.org/licenses/agpl-3.0)

[![Change Log](https://img.shields.io/badge/CHANGELOG-0.1.0-limegreen?logo=markdown)](CHANGELOG.md)

> ## Disclaimer
> *All trademarks, third-party assets/logos, and brand names used in this repository/project are the property of their respective owners. 
> This project is an independent educational resource and is not sanctioned, sponsored, or managed by any third-party trademark holders.*


> [!NOTE]
> ##### **Cargo package**, which represent a concrete project. It can <span style="color: red;font-weight: bold;font-size: 1.08rem;">&#42;</span>contain:
> + Zero or more `binary crates` (can contain *unit test*)
> + Zero or one `library crate` (can contain *unit test*)
> + Zero or more `bench` | `integration test`
> + However, a package must contain at least one crate; either a `library crate`, a `binary crate`, or both

## Overview Rust/Cargo project structure hierarchy
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
<br/>
<div
  class="
    display: flex;
    justify-content: flex-start;
  ">
  <img
      src="https://img.shields.io/badge/Download_Locally-gray?logo=git"
      style="
        display: flex;
        height:40px;
        padding-bottom: 4px;
      "
    /> 
</div>
<hr style="
  height:1.5px; 
  background-color:white; 
  border:none;">

### Clone
```bash
somewhere_dir > git clone [-b <remote_branch_name> --single-branch] [--no-tags] [--depth=<n gt 0>] --recursive <repo_url> [<destination_filepath>] 
somewhere_dir > cd <the_project>

#REM: If you already clone it, but, without the '--recursive' or without the submodule
somewhere_dir > cd <the_project>
the_project > git submodule update --init --recursive
```

### About branches and tags
```bash
#REM: Show a list
the_project > git branch [-v] [-a | -r]
the_project > git tag [-l]
```
```bash
#REM: Fetching remote branch and/or tag
the_project > git remote set-branches --add origin <remote_branch_name> #REM: For branch persistent
the_project > git fetch origin [<remote_branch_name>...] [tag <remote_tag_name>...]
```
