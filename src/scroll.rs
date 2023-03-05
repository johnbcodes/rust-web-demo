use crate::people;

use askama::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use sqlx::SqlitePool;
use std::time::Instant;
use tracing::info;

pub(crate) async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "scroll/index.html")]
struct IndexTemplate {}

pub(crate) async fn page(
    pagination: Option<Query<people::Pagination>>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let start = Instant::now();
    let Query(pagination) = pagination.unwrap_or_default();
    let records = people::load(&pool, &pagination).await;
    let duration = start.elapsed().as_micros();
    info!("DB duration: {duration} μs");
    let total = records.len();
    info!("Records: ({total})");
    PageTemplate {
        pagination,
        records,
    }
}

#[derive(Template)]
#[template(path = "scroll/page.html")]
struct PageTemplate {
    pagination: people::Pagination,
    records: Vec<people::Person>,
}
