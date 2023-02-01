## Getting Started

### Without Docker

* Install dependencies `cargo install`
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
  * Invoke NPM from cargo (build.rs)
    * [Inspiration #1](https://github.com/koute/bytehound/blob/master/server-core/build.rs)
    * [Inspiration #2](https://github.com/davidpdrsn/axum-live-view/blob/main/xtask/src/main.rs)
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