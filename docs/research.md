## Further Research
* Docker
    * Image naming according to target environment
    * Image tagging with arguments or environment variables
    * Environment/image considerations
        * Debugging
        * Performance optimizations
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
    * ~~Invoke NPM from cargo (build.rs)~~ [On hold](#invoking-npm-from-cargo)
        * ~~[Inspiration](https://github.com/koute/bytehound/blob/master/server-core/build.rs)~~
    * Or... create separate binary that controls npm and then invokes Cargo as a library..
        * https://docs.rs/cargo/latest/cargo/
        * Still have to manually check for transitive JS changes..
* Operations
    * https://12factor.net/
    * Deployment
    * Observability
        * Measurement
            * Errors
            * Performance
            * Value stream


### Invoking NPM from Cargo

As of 2023/03/21 there's no way to use Cargo's file watching (using mtime) with NPM
to tell if any dependencies have changed. NPM rewrites package-lock.json even if no
change occurs. Even a few package directories inside of node_modules get touched
during the process so watching that directory does not work.

Taking the idea even further still, there's now way to tell when transitive dependencies
need updating.

Will revisit once [Orogene](https://github.com/orogene/orogene), hopefully, becomes more mature and feature complete 
