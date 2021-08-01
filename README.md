# Gubernator

[![Build](https://github.com/projectgubernator/gubernator/actions/workflows/build-rust.yaml/badge.svg)](https://github.com/projectgubernator/gubernator/actions/workflows/build-rust.yaml)

**Gubernator is an experimental container orchestration system.** It is an answer to the age-old
question: _"What if Kubernetes was built in an alternate reality where security and usability were
important?"_

> There is obviously not much of anything to see here, yet, and there probably won't be for a
> while. If you are interested in getting involved, shoot me a message on
> [twitter](https://twitter.com/sjroot).

## Overview

### Project Structure

This repository is organized as a [Cargo workspace](cargo-workspaces).

| Crate                 | Description                                                                            |
| --------------------- | -------------------------------------------------------------------------------------- |
| `gubernator`          | Facade library crate for all other crates, and binary crate for the Gubernator CLI.    |
| `gubernator_cri`      | Gubernator client for container runtimes implementing the Container Runtime Interface. |
| `gubernator_manifest` | Utilities for loading and interacting with Gubernator manifest files.                  |
| `gubernator_state`    | Gubernator client for managing cluster state. Currently provided exclusively by etcd.  |

[cargo-workspaces]: https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
