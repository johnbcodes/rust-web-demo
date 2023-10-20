# Start with the more complicated docker build image
FROM ghcr.io/johnbcodes/node-rust:current-1.73.0 as base

# we need rsync
RUN set -eux; \
    apt-get update; \
    apt-get install -y --no-install-recommends rsync; \
    apt-get clean; \
    rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*;

RUN mkdir -p /data

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over infrequently changing files
COPY package.json package-lock.json Cargo.lock Cargo.toml ./
# copy your source tree, ordered again by infrequent to frequently changed files
COPY tailwind.config.js ./
COPY build.rs ./
COPY ./migrations ./migrations
COPY ./ui ./ui
COPY ./src ./src

## Debug build
FROM base as debug

# Cache dependencies on subsequent builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/app/.npm \
    npm set cache /app/.npm && \
    npm install && \
    npm run build && \
    cargo install --debug --path .

## Deploy locally
FROM debug as dev

ENV DATABASE_URL=sqlite://data/demo.db

EXPOSE 8080

ENTRYPOINT ["demo"]

## Release build
FROM base as release

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/app/.npm \
    npm set cache /app/.npm && \
    npm ci && \
    npm run build && \
    cargo build --release && \
    cargo install --path .


# Can't use "scratch". By default Rust dynamically links to C libraries, https://bxbrenden.github.io/
# Compiling with musl has it's own complications, https://github.com/emk/rust-musl-builder/issues
FROM debian:bookworm-slim as deploy

WORKDIR /

RUN mkdir data

COPY --from=release /usr/local/cargo/bin/demo .

ENV DATABASE_URL=sqlite://data/demo.db

EXPOSE 8080

ENTRYPOINT ["/demo"]