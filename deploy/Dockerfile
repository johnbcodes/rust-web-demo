# Start with the more complicated docker build image
FROM ghcr.io/johnbcodes/node-rust:current-1.69.0 as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over slower changing files
COPY package.json package-lock.json Cargo.lock Cargo.toml ./
COPY ./public/favicon.ico ./public/favicon.ico

# Cache dependencies on subsequent builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/app/.npm \
    npm set cache /app/.npm && \
    npm ci && \
    cargo build --release && \
    rm src/*.rs

# copy your source tree
COPY tailwind.config.js ./
COPY build.rs ./
COPY ./migrations ./migrations
COPY ./ui ./ui
COPY ./src ./src

# build for release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/app/.npm \
    npm run build && \
    cargo install --path .

ENTRYPOINT ["demo"]

# Can't use "scratch". By default Rust dynamically links to C libraries, https://bxbrenden.github.io/
# Compiling with musl has it's own complications, https://github.com/emk/rust-musl-builder/issues
FROM debian:stable-slim as prod

WORKDIR /

RUN mkdir data

COPY --from=build /usr/local/cargo/bin/demo .

ENV DATABASE_URL=sqlite://data/demo.db

EXPOSE 8080

ENTRYPOINT ["/demo"]