use crate::{layout::Layout, people};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::time::Instant;
use tracing::info;

pub(crate) async fn index() -> impl IntoResponse {
    let template = Layout {
        head: markup::new! {
            title { "People Infinite Scroll" }
        },
        body: Index {},
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
    Index() {
        div[class="flex w-screen h-screen p-10"] {
            div[class="flex flex-col w-full border-t border-r border-black", "data-controller"="hello"] {
                div[class="flex flex-shrink-0 bg-black text-white"] {
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "ID" } }
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "Quote" } }
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "Created At" } }
                }
                div[class="overflow-auto"] {
                    $"turbo-frame"[id="people_1", src="/infinite-scroll/page"] {
                        div[class="flex flex-shrink-0"] {
                            div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] {
                                span { "This message will be replaced by the response from /people." }
                            }
                        }
                    }
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
        $"turbo-frame"[id={format!("people_{}", pagination.page)}] {
            @for record in *records {
                div[class="flex flex-shrink-0"] {
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
            @match pagination.next_page(records) {
                Some(next_page) => {
                    @let source = format!("/infinite-scroll/page?page={}&per_page={}", next_page, pagination.per_page);
                    $"turbo-frame"[id={format!("people_{}", next_page)}, loading="lazy", src=source] {}
                }
                None => {}
            }
        }
    }
}
