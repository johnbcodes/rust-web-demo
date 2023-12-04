use crate::{
    layout::Layout,
    people,
    people::{model::SearchResult, Pagination},
};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use serde::Deserialize;
use std::time::Instant;
use tracing::info;

pub(crate) async fn index() -> impl IntoResponse {
    let template = Layout {
        head: markup::new! {
            title { "Typeahead Search Example" }
        },
        body: Index {},
    };

    Html(template.to_string())
}

pub(crate) async fn results(
    query: Query<Submission>,
    State(pool): State<Pool<ConnectionManager<SqliteConnection>>>,
) -> impl IntoResponse {
    let Query(submission) = query;
    if !submission.query.is_empty() {
        let pagination = Pagination {
            page: 1,
            per_page: 10,
            search: Some(submission.query.clone()),
        };

        let start = Instant::now();
        let results = people::perform_search(&pool, &pagination).await;
        let count = results.len();
        let duration = start.elapsed().as_micros();
        info!("DB duration for {count} record(s): {duration} μs");
        Html(Results { results: &results }.to_string())
    } else {
        Html("".to_string())
    }
}

#[derive(Deserialize, Debug)]
pub struct Submission {
    query: String,
}

markup::define! {
    Index() {
        main[class="mt-1 ml-4"] {
            h3 { "Search People" }
            input[id="search_query",
                class="transition ease-in-out mt-3 py-1.5 px-3 text-base font-normal text-[#212529] bg-clip-padding border-[1px] border-[#ced4da] appearance-none rounded focus:color-[#212529] focus:border-[#86b7fe] focus:outline-none focus:outline-0 focus:shadow-[0_0_0_0.25rem_rgba(13,110,253,0.25)]",
                name="query",
                "type"="search",
                placeholder="Begin typing to search people...",
                pattern=".*\\w+.*",
                required,
                autocomplete="off",
                "hx-get"="/typeahead-search/results",
                "hx-trigger"="input changed delay:500ms, search",
                "hx-target"="#search_results"] {}
                div[id="search_results"] {}
        }
    }

    Results<'a>(results: &'a Vec<SearchResult>) {
        h1[class="mt-3 font-bold"] { "Results" }

        ul[class="mt-1", role="listbox"] {
            @for result in *results {
                li {
                    a[id={format!("search_result_{}", result.id)},
                        href="",
                        role="option",
                        class="aria-selected:outline-black"] {
                            @markup::raw(&result.name)
                    }
                }
            }
        }
    }
}
