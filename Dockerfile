# The official Rust image provides the compiler and Cargo toolchain.
# The non-slim image includes curl, which is used to install Vite+.
FROM rust:latest AS base

ENV VP_HOME=/root/.vite-plus \
    VP_VERSION=0.2.6 \
    PATH=/root/.vite-plus/bin:$PATH

RUN curl -fsSL https://vite.plus | bash

RUN mkdir -p /data

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over infrequently changing files
COPY build.rs ./
COPY Rocket.toml ./
COPY rust-toolchain.toml ./
COPY package.json package-lock.json Cargo.lock Cargo.toml vite.config.mjs ./
# copy your source tree, ordered again by infrequent to frequently changed files
COPY ./migrations ./migrations
COPY ./ui ./ui
COPY ./src ./src

## Debug build
FROM base AS debug

# Cache dependencies on subsequent builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    vp install --frozen-lockfile && \
    vp build && \
    cargo build && \
    install -Dm755 target/debug/demo /usr/local/cargo/bin/demo

## Deploy locally
FROM debug AS dev

ENV ROCKET_PROFILE=docker

EXPOSE 8080

ENTRYPOINT ["demo"]

## Release build
FROM base AS release

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    vp install --frozen-lockfile && \
    vp build && \
    cargo build --release && \
    install -Dm755 target/release/demo /usr/local/cargo/bin/demo

# Can't use "scratch". By default Rust dynamically links to C libraries, https://bxbrenden.github.io/
# Compiling with musl has it's own complications, https://github.com/emk/rust-musl-builder/issues
FROM debian:stable-slim AS deploy

WORKDIR /

RUN mkdir data

COPY --from=release /app/Rocket.toml .
COPY --from=release /usr/local/cargo/bin/demo .

ENV ROCKET_PROFILE=docker

EXPOSE 8080

ENTRYPOINT ["/demo"]
