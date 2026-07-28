## Getting Started

### Without Docker

#### Prerequisites

* Rust version 1.78.0 or greater installed
* Vite+ installed (`vp`; see https://viteplus.dev/guide/)
* Node.js 24 (managed automatically by Vite+)

#### Install and build

* Install frontend dependencies `vp install`
* Build web with `vp build` (or `npm run build`)
* Install Rust dependencies `cargo install`
* Build with `cargo build`
* Run with `cargo run`

### With Docker

#### Prerequisite

* Docker and Docker Compose or compatible software installed.

#### Docker only

* Create volume with `docker volume create db-data`
* Build with `docker build -t rust-web-demo .`
* Run with `docker run -itd -p 8080:8080 -v db-data:/data rust-web-demo`

#### Docker Compose

* Build with `docker compose build`
* Run with `docker compose up` or `docker compose up -d` (build step not necessary once built)
