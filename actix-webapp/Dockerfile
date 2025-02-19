ARG RUST_VERSION=1.84.1
ARG APP_NAME=actix-webapp

FROM rust:${RUST_VERSION}-bullseye as builder
ARG APP_NAME
WORKDIR /app

################################################################################
# Create a DEV stage for building the application.
FROM builder AS dev

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y

RUN cargo install cargo-watch

# Install rustfmt
RUN rustup component add rustfmt