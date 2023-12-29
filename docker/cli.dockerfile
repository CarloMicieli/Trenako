FROM rust:1.75.0-slim-bookworm@sha256:90a3721bb0a8f79ab4a4cb20e7f78c050c9cc253908f704b8b49d0427818b8f5 as builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY crates/ ./crates/

RUN cargo build --release --bin trenako-cli

FROM debian:bookworm-slim@sha256:93ff361288a7c365614a5791efa3633ce4224542afb6b53a1790330a8e52fc7d as runtime
LABEL maintainer="Carlo Micieli <mail@trenako.com>"
LABEL description="The trenako command line interface"

ARG APP=/usr/src/app
ARG USERNAME=migrations
ARG USER_UID=1001
ARG USER_GID=$USER_UID

RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME}

COPY --from=builder /app/target/release/trenako-cli ${APP}/trenako-cli

USER ${USERNAME}

WORKDIR ${APP}
ENTRYPOINT ["/usr/src/app/trenako-cli"]
