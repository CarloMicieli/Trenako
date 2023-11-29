FROM rust:1.74.0-bookworm@sha256:e0740b99be8107433f623772f3ee44f63da96df56306a8b599bfd501a256e9b9 as builder
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
