FROM rust:1.73 as builder
WORKDIR /app
COPY ./migrations ./migrations

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

FROM debian:bullseye-slim as runtime

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

USER ${USERNAME}

WORKDIR ${APP}
CMD ["./sqlx", "migrate", "run"]
