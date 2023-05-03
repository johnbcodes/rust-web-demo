# Start with the more complicated docker build image
FROM ghcr.io/johnbcodes/node-rust:current-1.69.0 as build

RUN mkdir -p /data

# create a new empty shell project
RUN USER=root cargo new --bin app
WORKDIR /app

# copy over slower changing files
COPY package.json package-lock.json Cargo.lock Cargo.toml ./
# copy your source tree
COPY tailwind.config.js ./
COPY build.rs ./
COPY ./migrations ./migrations
COPY ./ui ./ui
COPY ./src ./src

# Cache dependencies on subsequent builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    --mount=type=cache,target=/app/.npm \
    npm set cache /app/.npm && \
    npm install && \
    npm run build && \
    cargo install --debug --path .

ENV DATABASE_URL=sqlite://data/demo.db

EXPOSE 8080

ENTRYPOINT ["demo"]