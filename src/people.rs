use askama::Template;
use axum::{
    extract::{Extension, Query},
    response::IntoResponse
};
use serde::Deserialize;
use sqlx::{PgPool, query_as};
use tracing::info;

#[derive(Debug)]
pub(crate) struct Person {
    pub(crate) id: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String
}

async fn load(pool: &PgPool, pagination: &Pagination) -> Vec<Person> {
    info!("Pagination: {:?}", pagination);
    if let Some(search) = &pagination.search {
        perform_search(pool, pagination, search).await
    } else {
        just_page(pool, pagination).await
    }
}

async fn perform_search(pool: &PgPool, pagination: &Pagination, search: &String) -> Vec<Person> {
    let search = format!("%{search}%");
    query_as!(
        Person,
        "select * from people where (first_name ilike $1 or last_name ilike $1) order by last_name, first_name limit $2 offset $3",
        search,
        pagination.per_page,
        pagination.offset()
    )
        .fetch_all(pool)
        .await
        .unwrap()
}

async fn just_page(pool: &PgPool, pagination: &Pagination) -> Vec<Person> {
    query_as!(
            Person,
            "select * from people order by last_name, first_name limit $1 offset $2",
            pagination.per_page,
            pagination.offset()
        )
        .fetch_all(pool)
        .await
        .unwrap()
}

pub(crate) async fn people(pagination: Option<Query<Pagination>>, Extension(pool): Extension<PgPool>) -> impl IntoResponse  {
    let Query(pagination) = pagination.unwrap_or_default();
    let records = load(&pool, &pagination).await;
    let total = records.len();
    info!("Records: ({total})");
    PeopleTemplate{ pagination, records }
}

#[derive(Template)]
#[template(path = "people.html")]
struct PeopleTemplate {
    pagination: Pagination,
    records: Vec<Person>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Pagination {
    page: i64,
    per_page: i64,
    search: Option<String>
}

impl Pagination {
    fn start(&self) -> i64 {
        (self.page - 1) * self.per_page
    }

    fn offset(&self) -> i64 {
        self.start() + self.per_page
    }

    fn next_page(&self, records: &Vec<Person>) -> Option<i64> {
        if (records.len() as i64) == self.per_page { Some(self.page + 1) } else { None }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self { page: 1, per_page: 30, search: None }
    }
}