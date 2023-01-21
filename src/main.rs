#![warn(clippy::all)]
#![deny(unreachable_pub, private_in_public)]
#![forbid(unsafe_code)]

mod assets;
mod people;
mod scroll;
mod typeahead;

use askama::Template;
use assets::asset_handler;
use axum::{
    handler::HandlerWithoutStateExt,
    response::IntoResponse,
    routing::{get, Router},
};
use dotenvy::dotenv;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous};
use std::env;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // WARNING: Do not initialize logging before running migrations. Large migrations run slowly
    // due to reformatting the SQL statement(s)

    let db_url = env::var("DATABASE_URL").unwrap();
    let options = SqliteConnectOptions::from_str(db_url.as_str())
        .unwrap()
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(10)
        .connect_with(options)
        .await
        .expect("unable to connect to database");

    sqlx::migrate!().run(&pool).await.unwrap();

    fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/", get(directory))
        .route("/typeahead-search", get(typeahead::index))
        .route("/typeahead-search/results", get(typeahead::results))
        .route("/infinite-scroll", get(scroll::index))
        .route("/infinite-scroll/page", get(scroll::page))
        .route_service("/dist/*file", asset_handler.into_service())
        .with_state(pool)
        .layer(TraceLayer::new_for_http())
        .fallback_service(asset_handler.into_service());

    let addr: SocketAddrV4 = "0.0.0.0:3000".parse().unwrap();
    let socket = SocketAddr::from(addr);
    info!("listening on {socket}");
    axum::Server::bind(&socket)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn directory() -> impl IntoResponse {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "directory.html")]
struct IndexTemplate {}
