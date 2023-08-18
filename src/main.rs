#![warn(clippy::all)]
#![deny(unreachable_pub, private_in_public)]
#![forbid(unsafe_code)]

mod assets;
mod layout;
mod migrations;
mod people;
mod scroll;
mod typeahead;

// Required because of visibility of generated structs by markup.rs
pub use people::{Pagination, Person, SearchResult};
pub use typeahead::Submission;

use assets::asset_handler;
use axum::response::Html;
use axum::{
    handler::HandlerWithoutStateExt,
    response::IntoResponse,
    routing::{get, Router},
};
use dotenvy::dotenv;
use layout::Layout;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::OpenFlags as of;
use std::env;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tower_http::LatencyUnit;
use tracing::{info, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

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

    let db_url = env::var("DATABASE_FILE").unwrap();
    println!("DATABASE_FILE={db_url}");

    let manager = SqliteConnectionManager::file(db_url.as_str())
        .with_flags(of::SQLITE_OPEN_URI | of::SQLITE_OPEN_CREATE | of::SQLITE_OPEN_READ_WRITE)
        .with_init(|conn| conn.pragma_update(None, "journal_mode", "wal"))
        .with_init(|conn| conn.pragma_update(None, "synchronous", "normal"))
        .with_init(|conn| conn.pragma_update(None, "foreign_keys", "on"));
    let pool = Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("unable to build pool");

    let mut conn = pool.get().unwrap();
    migrations::MIGRATIONS.to_latest(&mut conn).unwrap();
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
