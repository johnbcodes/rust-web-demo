# Start with the more complicated docker build image
FROM ghcr.io/johnbcodes/node-rust:current-1.66.1 as build

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

RUN mkdir db

# copy over slower changing files
COPY package.json package-lock.json Cargo.lock Cargo.toml ./
COPY ./public/favicon.ico ./public/favicon.ico
# copy your source tree
COPY tailwind.config.js ./
COPY ./src ./src
COPY ./templates ./templates
COPY ./migrations ./migrations

# Cache dependencies on subsequent builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/app/.npm \
    npm set cache /app/.npm && \
    npm install && \
    npm run build && \
    cargo install --debug --path .

ENTRYPOINT ["demo"]