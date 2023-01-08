FROM rust:1.66 as planner
WORKDIR /app
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef 

COPY Cargo.toml .
COPY Cargo.lock .
COPY crates/ ./crates/

RUN cargo chef prepare  --recipe-path recipe.json

FROM rust:1.66 as cacher
WORKDIR /app

RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust:1.66 as builder
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY crates/ ./crates/
COPY config/ ./config/

# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin trenako-server

FROM debian:buster-slim as runtime
LABEL maintainer="Carlo Micieli <mail@trenako.com>"
LABEL description="The trenako backend component"

ARG APP=/usr/src/app

HEALTHCHECK --interval=5m --timeout=3s \
  CMD curl -f http://localhost/health-check || exit 1

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata curl \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP} \
    && mkdir -p ${APP}/config

COPY --from=builder /app/target/release/trenako-server ${APP}/trenako-server
COPY --from=builder /app/config/application-docker.yml ${APP}/config/application.yml

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./trenako-server"]
