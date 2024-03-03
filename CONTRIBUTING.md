# Contribute

- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [First setup](#first-setup)
  - [Optional requirements](#optional-requirements)
  - [Test](#test)
- [Code of Conduct](#code-of-conduct)
- [Commit Message Format](#commit-message-format)

---

## Getting Started

### Requirements

- [Rust](https://www.rust-lang.org/tools/install): v1

### First setup

> [!IMPORTANT]  
> If you're under **Windows**, you nust run all CLI commands under a Linux shell-like terminal (i.e.: WSL or Git Bash).

Then run:

```sh
git clone https://github.com/ivangabriele/mistralai-client-rs.git # or your fork
cd ./mistralai-client-rs
cargo build
```

### Optional requirements

- [cargo-watch](https://github.com/watchexec/cargo-watch#install) for `make test-*-watch`.

### Test

```sh
make test
```

or

```sh
make test-watch
```

## Code of Conduct

Help us keep this project open and inclusive. Please read and follow our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Commit Message Format

This repository follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification and
specificaly the [Angular Commit Message Guidelines](https://github.com/angular/angular/blob/main/CONTRIBUTING.md#commit).