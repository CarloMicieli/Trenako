FROM rust:1.66 as builder
WORKDIR /app
COPY ./migrations ./migrations

RUN cargo install sqlx-cli --no-default-features --features native-tls,postgres

FROM debian:buster-slim as runtime
FROM gcc
ARG APP=/usr/src/app

COPY --from=builder /usr/local/cargo/bin/sqlx ${APP}/sqlx
COPY --from=builder /app/migrations ${APP}/migrations

RUN apt-get update && apt-get -y install libssl-dev

WORKDIR ${APP}
RUN ./sqlx -V

CMD ["./sqlx", "migrate", "run"]
