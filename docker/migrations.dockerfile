FROM rust:1.75.0-bookworm@sha256:85dcf8f6723b3045a200d1b5ac60f75bb26efe43788071d5f838a35e10003af2 as builder
WORKDIR /app
COPY ./migrations ./migrations

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

FROM debian:bookworm-slim@sha256:45287d89d96414e57c7705aa30cb8f9836ef30ae8897440dd8f06c4cff801eec as runtime

LABEL maintainer="Carlo Micieli <mail@trenako.com>"
LABEL description="The trenako database migrations"

ARG APP=/usr/src/app
ARG USERNAME=migrations
ARG USER_UID=1001
ARG USER_GID=$USER_UID

RUN groupadd --gid ${USER_GID} ${USERNAME} \
    && useradd --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME}

COPY --from=builder /usr/local/cargo/bin/sqlx ${APP}/sqlx
COPY --from=builder /app/migrations ${APP}/migrations

RUN apt-get update && apt install -y openssl

USER ${USERNAME}

WORKDIR ${APP}
CMD ["./sqlx", "migrate", "run"]
