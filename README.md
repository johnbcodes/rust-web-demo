## Getting Started

### Without Docker

#### Prerequisites

* Rust version 1.78.0 or greater installed
* NodeJS version 20 or greater installed

#### Install and build

* Install Node dependencies `npm install`
* Build web with `npm run build`
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

## Initial deployment to fly.io with `flyctl` (aliased to `fly`)
* Create account if necessary
* `fly auth login`
* `fly apps create <GLOBALLY-UNIQUE-APP-NAME>`
  * Update `app` property in `fly.toml` with <APP-NAME>
* Choose fly.io region
  * Update `primary_region` property in `fly.toml`
* `fly volumes create <VOLUME-NAME> -s 1 -r <REGION>`
  * Update `mounts.source` property in `fly.toml` with <VOLUME-NAME>
* `docker build -t registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER> --target deploy .`
* `fly deploy --image registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`

## Automated deployment of new versions with GitHub [action](.github/workflows/deploy.yml)
* [Set up](https://docs.github.com/en/actions/security-guides/using-secrets-in-github-actions) your `FLY_API_TOKEN` [secret](https://fly.io/docs/reference/deploy-tokens/) in your repository
* Tag release with a tag name starting with 'v'
  * Example: `git tag -a v2 -m "My new release!" && git push --tags`

## Manual deployment from local image
* `docker build -t registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER> --target deploy .`
* `fly auth docker`
* `docker push registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`
* `fly deploy --image registry.fly.io/<GLOBALLY-UNIQUE-APP-NAME>:<VERSION-NUMBER>`
