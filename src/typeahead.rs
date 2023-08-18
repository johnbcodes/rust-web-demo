use crate::{
    layout::Layout,
    people,
    people::{Pagination, SearchResult},
};
use axum::{
    extract::{Query, State},
    response::{Html, IntoResponse},
};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
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

    Html(
        Results {
            submission: &submission,
            results: &results,
        }
        .to_string(),
    )
}

#[derive(Deserialize, Debug)]
pub struct Submission {
    query: String,
    turbo_frame: String,
}

markup::define! {
    Index() {
        header["data-controller"="typeahead--combobox"] {
            form[action="/typeahead-search/results",
                "data-turbo-frame"="search_results",
                "data-controller"="typeahead--search",
                "data-action"="invalid->typeahead--search#hideValidationMessage:capture input->typeahead--search#submit",
                class="peer"] {

                label["for"="search_query"] { "Query" }
                input[id="search_query",
                    name="query",
                    "type"="search",
                    pattern=".*\\w+.*",
                    required,
                    autocomplete="off",
                    "data-typeahead--combobox-target"="input",
                    "data-action"="focus->typeahead--combobox#start focusout->typeahead--combobox#stop"] {}
                input[id="turbo_frame", "type"="hidden", name="turbo_frame", value="search_results"] {}
                button["data-typeahead--search-target"="submit"]{
                    "Search"
                }
                $"turbo-frame"[id="search_results", target="_top", class="empty:hidden peer-invalid:hidden"] {}
            }
        }
        main {

        }
    }

    Results<'a>(submission: &'a Submission, results: &'a Vec<SearchResult>) {
        $"turbo-frame"[id={&submission.turbo_frame}] {
            h1 { "Results" }

            ul[role="listbox", "data-typeahead--combobox-target"="list"] {
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
}
