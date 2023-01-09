# Contribution guidelines

First off, thank you for considering contributing to trenako.

If your contribution is not straightforward, please first discuss the change you
wish to make by creating a new issue before making the change.

## Reporting issues

Before reporting an issue on the
[issue tracker](https://github.com/CarloMicieli/trenako/issues),
please check that it has not already been reported by searching for some related
keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the changes you have made in
[CHANGELOG](https://github.com/CarloMicieli/trenako/blob/main/CHANGELOG.md)
file under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections,
depending on the types of changes defined by
[Keep a changelog](https://keepachangelog.com/en/1.0.0/):

- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it!

## Developing

### Set up

* Install the 🦀 rust toolchain in order to have cargo installed by
  following [this](https://www.rust-lang.org/tools/install) guide.

```shell
  cargo install cargo-tarpaulin
  cargo install cargo-audit
  cargo install sqlx-cli --no-default-features --features native-tls,postgres
  rustup component add clippy
  rustup component add rustfmt
```

```shell
  git clone https://github.com/CarloMicieli/trenako
  cd trenako
  cargo test
```

### Useful Commands

| Command                          | Description                        |
|----------------------------------|------------------------------------|
| `cargo run`                      | run the app                        |
| `cargo test`                     | run the tests                      |
| `cargo fmt -- --check`           | check the formatting               |
| `cargo clippy`                   | run the linter                     |
| `cargo tarpaulin --ignore-tests` | compute code coverage              |
| `cargo audit`                    | check for security warnings        |
| `cargo sqlx prepare --merged`    | prepare the query for offline mode |
| `sqlx migrate add <name>`        | creates a new `<name>` migration   |
| `sqlx migrate run`               | run the database migrations        |

### Conventional commits

This repository is following the conventional commits practice.

#### Enforcing using git hooks

```shell
  git config core.hooksPath .githooks
```

The hook itself can be found in `.githooks/commit-msg`.

#### Using Commitizen

Install [commitizen](https://github.com/commitizen-tools/commitizen)

```shell
  pip install commitizen
```

and then just use it

```shell
  cz commit
```
