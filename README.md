## Getting Started

### Without Docker

#### Prerequisites

* Rust version 1.62.1 or greater installed
* NodeJS version 18 or greater installed

* Install Node dependencies `npm install`
* Build with `npm run build` 
* Install Rust dependencies `cargo install`
* Build with `cargo build`
* Run with `cargo run`

### With Docker

#### Prerequisite

* Docker and Docker Compose or compatible software installed.

#### Docker only

* Create volume with `docker volume create db-data`
* Build with `docker build -t rust-web-demo .`
* Run with `docker run -itd -e "DATABASE_URL=sqlite:///data/demo.db" -p 8080:8080 -v db-data:/data rust-web-demo`

#### Docker Compose

* Build with `docker compose build`
* Run with `docker compose up` or `docker compose up -d` (build step not necessary)

## Deploying to Fly.io

* Create account
* `fly auth login`
* `fly apps create`
  * Update `app` property in `fly.toml` with app name 
* `fly volumes create <volume-name> --size 1`
  * Update `mounts.source` property in `fly.toml` with mount name
* `fly secrets set DATABASE_URL=sqlite3:///data/demo.db`
* `fly deploy`


[Markdown Cheatsheet](https://www.markdownguide.org/cheat-sheet/)

[Conventional Commits Cheatsheet](https://cheatography.com/albelop/cheat-sheets/conventional-commits/)

~~Strikethrough~~