FROM rust:1.75.0-bookworm@sha256:85dcf8f6723b3045a200d1b5ac60f75bb26efe43788071d5f838a35e10003af2 as builder
WORKDIR /app
COPY ./migrations ./migrations

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

FROM debian:bookworm-slim@sha256:93ff361288a7c365614a5791efa3633ce4224542afb6b53a1790330a8e52fc7d as runtime

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
