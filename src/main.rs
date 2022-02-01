#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::mismatched_target_os,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    // missing_docs
)]
#![deny(unreachable_pub, private_in_public)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity)]
#![forbid(unsafe_code)]

mod assets;
mod people;
mod scroll;
mod typeahead;

use askama::Template;
use assets::asset_handler;
use axum::{
    handler::Handler,
    response::IntoResponse,
    routing::get,
    Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tower_http::{add_extension::AddExtensionLayer, trace::TraceLayer};
use tracing::{info, Level};
use tracing_subscriber::fmt;

#[tokio::main]
async fn main() {
    dotenv().ok();
    fmt().with_max_level(Level::INFO).init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(env::var("DATABASE_URL").unwrap().as_str()).await.unwrap();

    let app = Router::new()
        .route("/", get(directory))
        .route("/typeahead-search", get(typeahead::index))
        .route("/typeahead-search/results", get(typeahead::results))
        .route("/infinite-scroll", get(scroll::index))
        .route("/infinite-scroll/page", get(scroll::page))
        .route("/dist/*rest", asset_handler.into_service())
        .layer(AddExtensionLayer::new(pool))
        .layer(TraceLayer::new_for_http())
        .fallback(asset_handler.into_service());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn directory() -> impl IntoResponse {
    IndexTemplate{}
}

#[derive(Template)]
#[template(path = "directory.html")]
struct IndexTemplate {}