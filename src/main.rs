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

use askama::Template;
use assets::asset_handler;
use axum::{
    extract::{Path, Query},
    handler::Handler,
    response::IntoResponse,
    routing::get,
    Router,
};
use fake::locales::EN;
use fake::Fake;
use fake::faker::name::raw::{FirstName, LastName};
use lazy_static::lazy_static;
use nanoid::nanoid;
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing::{info, Level};
use tracing_subscriber::fmt;
use voca_rs::case;

lazy_static! {
    static ref PEOPLE: Vec<(String, String, String)> = {
        let mut p = Vec::with_capacity(1000);
        for _n in 1..=1000 {
            p.push((nanoid!(), FirstName(EN).fake(), LastName(EN).fake()));
        }
        p
    };
}

#[tokio::main]
async fn main() {
    fmt().with_max_level(Level::INFO).init();

    let app = Router::new()
        .route("/", get(index))
        .route("/favicon.ico", asset_handler.into_service())
        .route("/people", get(people))
        .route("/greet/:name", get(greet))
        .route("/dist/*rest", asset_handler.into_service())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("listening on {addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn greet(Path(name): Path<String>) -> impl IntoResponse  {
    GreetTemplate{ name: case::capitalize(&name, false) }
}

#[derive(Template)]
#[template(path = "greet.html")]
struct GreetTemplate {
    name: String,
}

async fn index() -> impl IntoResponse  {
    IndexTemplate{}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}

async fn people(pagination: Option<Query<Pagination>>) -> impl IntoResponse  {
    let Query(pagination) = pagination.unwrap_or_default();
    info!("Query: {:?}", pagination);
    let end = if pagination.end() <= PEOPLE.len() { pagination.end() } else { PEOPLE.len() };
    let records = &PEOPLE[pagination.start()..end];
    PeopleTemplate{ pagination, records }
}

#[derive(Template)]
#[template(path = "people.html")]
struct PeopleTemplate<'a> {
    pagination: Pagination,
    records: &'a [(String, String, String)],
}

#[derive(Deserialize, Debug)]
struct Pagination {
    page: usize,
    per_page: usize,
}

impl Pagination {
    fn start(&self) -> usize {
        (self.page - 1) * self.per_page
    }

    fn end(&self) -> usize {
        self.start() + self.per_page
    }

    fn next_page(&self, records: &[(String, String, String)]) -> Option<usize> {
        if records.len() == self.per_page { Some(self.page + 1) } else { None }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, per_page: 30 }
    }
}