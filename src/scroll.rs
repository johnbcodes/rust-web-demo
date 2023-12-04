use crate::{layout::Layout, people};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::time::Instant;
use tracing::info;

pub(crate) async fn index(
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl IntoResponse {
    let start = Instant::now();
    let pagination = people::Pagination::default();
    let records = people::just_page(&pool, &pagination).await;
    let duration = start.elapsed().as_micros();
    info!("DB duration: {duration} μs");
    let template = Layout {
        head: markup::new! {
            title { "People Infinite Scroll" }
        },
        body: Index {
            pagination: &pagination,
            records: &records,
        },
    };

    Html(template.to_string())
}

pub(crate) async fn page(
    pagination: Option<Query<people::Pagination>>,
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl IntoResponse {
    let start = Instant::now();
    let Query(pagination) = pagination.unwrap_or_default();
    let records = people::just_page(&pool, &pagination).await;
    let duration = start.elapsed().as_micros();
    info!("DB duration: {duration} μs");
    let total = records.len();
    info!("Records: ({total})");

    Html(
        Pager {
            pagination: &pagination,
            records: &records,
        }
        .to_string(),
    )
}

markup::define! {
    Index<'a>(pagination: &'a people::Pagination, records: &'a Vec<people::model::Person>) {
        div[class="flex w-screen h-screen p-10"] {
            div[class="flex flex-col w-full border-t border-r border-black", "data-controller"="hello"] {
                div[class="flex flex-shrink-0 bg-black text-white"] {
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "ID" } }
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "Quote" } }
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "Created At" } }
                }
                div[class="overflow-auto"] {
                    @Pager{ pagination, records }
                }
                div[class="flex flex-shrink-0 bg-black text-white"] {
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "ID" } }
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "First Name" } }
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "Last Name" } }
                }
            }
        }
    }

    Pager<'a>(pagination: &'a people::Pagination, records: &'a Vec<people::model::Person>) {
        @for (i, record) in records.iter().enumerate() {
            @if i != records.len() - 1 {
                div[class="flex flex-shrink-0"] {
                    @Row{ record }
                }
            } else {
                @match pagination.next_page(records) {
                    Some(next_page) => {
                        @let get = format!("/infinite-scroll/page?page={}&per_page={}", next_page, pagination.per_page);
                        div[class="flex flex-shrink-0", "hx-get"=get, "hx-trigger"="intersect once", "hx-swap"="afterend"] {
                            @Row{ record }
                        }
                    }
                    None => {
                        div[class="flex flex-shrink-0"] {
                            @Row{ record }
                        }
                    }
                }
            }
        }
    }

    Row<'a>(record: &'a people::model::Person) {
        div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] {
            span { @record.id }
        }
        div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] {
            span { @record.first_name }
        }
        div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] {
            span { @record.last_name }
        }
    }
}
