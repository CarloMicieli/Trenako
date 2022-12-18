# trenako

![license](https://img.shields.io/github/license/CarloMicieli/trenako)
![GitHub last commit](https://img.shields.io/github/last-commit/CarloMicieli/trenako)
[![ci](https://github.com/CarloMicieli/trenako/actions/workflows/ci.yml/badge.svg)](https://github.com/CarloMicieli/trenako/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/CarloMicieli/trenako/branch/main/graph/badge.svg?token=i8xoC46ZYN)](https://codecov.io/gh/CarloMicieli/trenako)

A website for model railway collectors.

## Features

tbd

## Tech Stack

* 🦀 `Rust`
* `Cargo`
* `Docker` / `Docker compose`

## How to run

### Database

To run the `postgres` database:

```bash
  docker run -it --rm --name roundhouse-db-dev \
    -e POSTGRES_PASSWORD=mysecretpassword \
    -e POSTGRES_DB=roundhouse \
    -d -p 5432:5432 \
    -v roundhouse_db_dev:/var/lib/postgresql/data \
    postgres:14.5-alpine
```

### Start the server

```shell
  git clone https://github.com/CarloMicieli/trenako
  cd trenako
  cargo run --bin trenako

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

### Environment variables

| Variable      | Description            |
|---------------|------------------------|
| `SERVER_HOST` | the server host name   |
| `SERVER_PORT` | the server port number |

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
