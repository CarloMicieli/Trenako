# trenako

![license](https://img.shields.io/github/license/CarloMicieli/trenako)
![GitHub last commit](https://img.shields.io/github/last-commit/CarloMicieli/trenako)
[![CI (Rust)](https://github.com/CarloMicieli/trenako/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/CarloMicieli/trenako/actions/workflows/rust-ci.yml)
[![codecov](https://codecov.io/gh/CarloMicieli/trenako/branch/dev/graph/badge.svg?token=i8xoC46ZYN)](https://codecov.io/gh/CarloMicieli/trenako)

A website for model railway collectors.

## Features

tbd

## Tech Stack

* 🦀 `Rust`
* `Cargo` and `cargo-make`
* `Docker` / `Docker compose`

## How to run

```bash
  git clone https://github.com/CarloMicieli/trenako
  cd trenako

  docker compose up
```

- The database admin webpage is available at http://localhost:9000/
- The open api documentation is available at http://localhost:9001/

### Local development

All the most important tasks are defined in the `Makefile.toml` for `cargo-make`.

In order to install `cargo-make` just run the following command:

```bash
  cargo install --force cargo-make
```

To run the `postgres` database as detached docker container:

```bash
  cargo make docker-postgres-run
```

to execute the database migrations:

```bash
  cargo make db-migrate
```

The sqlx `query!` macro is checking the query commands against a live database, to avoid to fail the build when a database is not available the offline mode is handled saving the query information into a json file. To update the file run the following command:

```bash
  cargo make db-update-offline
```

### Start the server

```bash
  cargo make run

    Finished dev [unoptimized + debuginfo] target(s) in 0.10s
     Running `target/debug/trenako`

 _                        _         
| |                      | |        
| |_ _ __ ___ _ __   __ _| | _____  
| __| '__/ _ \ '_ \ / _` | |/ / _ \ 
| |_| | |  __/ | | | (_| |   < (_) |
 \__|_|  \___|_| |_|\__,_|_|\_\___/

Starting the server (127.0.0.1:9999)...
```

#### Environment variables

| Variable            | Description              |
|---------------------|--------------------------|
| `SERVER_HOST`       | the server host name     |
| `SERVER_PORT`       | the server port number   |
| `SERVER_WORKERS`    | the number of workers    |
| `DATABASE_NAME`     | the database name        |
| `DATABASE_USERNAME` | the database username    |
| `DATABASE_PASSWORD` | the database password    |
| `DATABASE_HOST`     | the database hostname    |
| `DATABASE_PORT`     | the database port number |

### Checks

Run all tests with the following command:

```bash
  cargo make test
```

The rust code is following the rust formatting standard (via `rustfmt`), to check if the formatting is correct:

```bash
  cargo make format
```

To run the rust linter (`clippy`):

```bash
  cargo make clippy
```

## Contribution

Contributions are always welcome!

See [CONTRIBUTING.md](CONTRIBUTING.md) for ways to get started.

Please adhere to this project's [code of conduct](CODE_OF_CONDUCT.md).

## License

[MIT License](https://choosealicense.com/licenses/mit/)

```
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```
