use crate::{
    people,
    people::{Pagination, SearchResult},
};
use askama::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Deserialize;
use std::time::Instant;
use tracing::info;

pub(crate) async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "typeahead/index.html")]
struct IndexTemplate {}

pub(crate) async fn results(
    query: Query<Submission>,
    State(pool): State<Pool<SqliteConnectionManager>>,
) -> impl IntoResponse {
    let Query(submission) = query;
    let pagination = Pagination {
        page: 1,
        per_page: 10,
        search: Some(submission.query.clone()),
    };

    let start = Instant::now();
    let results = people::perform_search(&pool, &pagination).await;
    let duration = start.elapsed().as_micros();
    info!("DB duration: {duration} μs");

    ResultsTemplate {
        submission,
        results,
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Submission {
    query: String,
    turbo_frame: String,
}

#[derive(Template)]
#[template(path = "typeahead/results.html")]
struct ResultsTemplate {
    submission: Submission,
    results: Vec<SearchResult>,
}
