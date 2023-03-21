## Getting Started

### Without Docker

#### Prerequisites

* Rust version 1.68 or greater installed
* NodeJS version XX or greater installed

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


## Further exploration
* Docker
  * Image naming according to target environment 
  * Image tagging with arguments or environment variables
  * Environment/image considerations
    * Debugging
    * Performance optimizations
    * Deploy size
    * Observability
    * Security
    * Deployment
* Better styling
  * directory
  * infinite scroll example
  * typeahead search example
* Examples
  * Form example
    * Validation
      * Error handling
    * Styling
    * Double submission prevention
  * EventStoreDB
  * Tooltips
* Application UI
  * 3 column layout
  * Navigation
  * Testing
    * WebDriver/thirty-four/chromiumoxide
* UX
  * Error handling / pages
    * Remove unwraps
  * Eventual Consistency
  * Tooltips
  * Navigation / Drawers
  * Modals
  * Keyboard navigation/shortcuts
* Cross-cutting concerns
  * Error handling
  * Timeouts
  * Caching
  * Credential handling
* Database(s)
  * Pooling configuration
* HTTP
  * Idempotency
* Security
  * Sessions ([async-session](https://github.com/http-rs/async-session)/[async-sqlx-session](https://github.com/jbr/async-sqlx-session))
    * Cookies
    * Expiration
  * AuthN (??) / AuthZ (Casbin)
  * https://securityheaders.com/
    * CSP nonces
  * Rate limiting
    * Load testing (Oha)
* Build
  * ~~Invoke NPM from cargo (build.rs)~~ [On hold](https://gist.github.com/johnbcodes/46ccd7a6bc8029ec98721decaf7cbea5)
    * ~~[Inspiration](https://github.com/koute/bytehound/blob/master/server-core/build.rs)~~
  * Or... create separate binary that controls npm and then invokes Cargo as a library..
    * https://docs.rs/cargo/latest/cargo/
    * Still have to manually check for transitive js changes..
* Operations
  * https://12factor.net/
  * Deployment
  * Observability
    * Measurement
      * Errors
      * Performance
      * Value stream
  * SQLite Resiliency
    * Single node - Litestream
    * Multi-node - LiteFS

[Markdown Cheatsheet](https://www.markdownguide.org/cheat-sheet/)

[Conventional Commits Cheatsheet](https://cheatography.com/albelop/cheat-sheets/conventional-commits/)

~~Strikethrough~~