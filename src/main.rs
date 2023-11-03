#![warn(clippy::all)]
#![deny(unreachable_pub, private_in_public)]
#![forbid(unsafe_code)]

mod assets;
mod diesel_ext;
mod layout;
mod people;
mod schema;
mod scroll;
mod typeahead;

//noinspection RsUnusedImport
// Required because of visibility of generated structs by markup.rs
pub use people::{
    model::{Person, SearchResult},
    Pagination,
};
pub use typeahead::Submission;

use assets::asset_handler;
use axum::response::Html;
use axum::{
    handler::HandlerWithoutStateExt,
    response::IntoResponse,
    routing::{get, Router},
};
use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use dotenvy::dotenv;
use layout::Layout;
use std::env;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[tokio::main]
async fn main() {
    dotenv().ok();

    let rust_log = env::var("RUST_LOG").unwrap_or_else(|_| {
        let value = "INFO,tower_http=info";
        env::set_var("RUST_LOG", value);
        value.into()
    });
    println!("RUST_LOG={rust_log}");

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(fmt::layer())
        .init();

    let database_url = env::var("DATABASE_URL").unwrap();
    println!("DATABASE_URL={database_url}");

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Could not build connection pool");

    let mut conn = pool.get().unwrap();

    conn.batch_execute("
        PRAGMA journal_mode = WAL;          -- better write-concurrency
        PRAGMA synchronous = NORMAL;        -- fsync only in critical moments
        PRAGMA wal_autocheckpoint = 1000;   -- write WAL changes back every 1000 pages, for an in average 1MB WAL file. May affect readers if number is increased
        PRAGMA wal_checkpoint(TRUNCATE);    -- free some space by truncating possibly massive WAL files from the last run.
        PRAGMA busy_timeout = 250;          -- sleep if the database is busy
        PRAGMA foreign_keys = ON;           -- enforce foreign keys
    ").unwrap();

    conn.run_pending_migrations(MIGRATIONS).unwrap();
    drop(conn);

    let trace_layer = TraceLayer::new_for_http().on_response(
        DefaultOnResponse::new()
            .level(Level::INFO)
            .latency_unit(LatencyUnit::Micros),
    );

    let app = Router::new()
        .route("/", get(directory))
        .route("/typeahead-search", get(typeahead::index))
        .route("/typeahead-search/results", get(typeahead::results))
        .route("/infinite-scroll", get(scroll::index))
        .route("/infinite-scroll/page", get(scroll::page))
        .route_service("/dist/*file", asset_handler.into_service())
        .with_state(pool)
        .layer(trace_layer)
        .fallback_service(asset_handler.into_service());

    let addr = "[::]:8080".parse().unwrap();
    info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn directory() -> impl IntoResponse {
    let template = Layout {
        head: markup::new! {
            title { "Demo Directory" }
        },
        body: markup::new! {
            main {
              ul {
                li { a[href="/typeahead-search"] { "Typeahead Searching with Turbo Frames and Stimulus controllers" } }
                li { a[href="/infinite-scroll"] { "Infinite Scroll with Turbo Frames" } }
              }
            }
        },
    };

    Html(template.to_string())
}
