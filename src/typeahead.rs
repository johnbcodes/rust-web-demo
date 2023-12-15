use crate::{
    layout::Layout,
    people,
    people::{model::SearchResult, Pagination},
    Db,
};
use rocket::{fairing::AdHoc, response::content::RawHtml};

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Typeahead Search Stage", |rocket| async {
        rocket.mount("/typeahead-search", routes![index, results])
    })
}

#[get("/")]
async fn index() -> RawHtml<String> {
    let template = Layout {
        head: markup::new! {
            title { "Typeahead Search Example" }
        },
        body: Index {},
    };

    RawHtml(template.to_string())
}

#[get("/results?<query>")]
async fn results(db: Db, query: String) -> RawHtml<String> {
    if !query.is_empty() {
        let pagination = Pagination {
            page: 1,
            per_page: 10,
            search: Some(query),
        };

        let results = db
            .run(move |conn| people::perform_search(conn, &pagination))
            .await;

        RawHtml(Results { results: &results }.to_string())
    } else {
        RawHtml("".to_string())
    }
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
