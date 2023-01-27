FROM rust:1.67 as builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY crates/ ./crates/

RUN cargo build --release --bin trenako-cli

FROM debian:bullseye-slim as runtime
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
