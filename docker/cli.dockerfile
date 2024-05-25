# syntax=docker/dockerfile:1
ARG RUST_VERSION=1.78.0
ARG RUST_IMAGE_SHA256=0fea967628dc796a2b9d1d57ddb3af3b3f0a35b6c8c0e23690dbe0ceb71a2dc9
ARG APP_NAME=trenako-cli
ARG RUNTIME_IMAGE_SHA256=45287d89d96414e57c7705aa30cb8f9836ef30ae8897440dd8f06c4cff801eec

################################################################################
# Create a stage for building the application.
FROM rust:${RUST_VERSION}-slim-bookworm@sha256:${RUST_IMAGE_SHA256} as build
ARG APP_NAME
WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY crates/ ./crates/

# Build the application.
# Leverage a cache mount to /usr/local/cargo/registry/
# for downloaded dependencies, a cache mount to /usr/local/cargo/git/db
# for git repository dependencies, and a cache mount to /app/target/ for
# compiled dependencies which will speed up subsequent builds.
# Leverage a bind mount to the src directory to avoid having to copy the
# source code into the container. Once built, copy the executable to an
# output directory before the cache mounted /app/target is unmounted.
RUN --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release --bin trenako-cli --target-dir ./target
cp ./target/release/$APP_NAME /bin/trenako-cli
EOF

################################################################################
# Create a new stage for running the application that contains the minimal
# runtime dependencies for the application. This often uses a different base
# image from the build stage where the necessary files are copied from the build
# stage.
#
# The example below uses the alpine image as the foundation for running the app.
# By specifying the "3.18" tag, it will use version 3.18 of alpine. If
# reproducability is important, consider using a digest
# (e.g., alpine@sha256:664888ac9cfd28068e062c991ebcff4b4c7307dc8dd4df9e728bedde5c449d91).
FROM debian:bookworm-slim@sha256:${RUNTIME_IMAGE_SHA256} as runtime
LABEL maintainer="Carlo Micieli <mail@trenako.com>"
LABEL description="The trenako command line interface"

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/trenako-cli /bin/

USER appuser

WORKDIR ${APP}
ENTRYPOINT ["/bin/trenako-cli"]
