use crate::{layout::Layout, people, Db};
use rocket::{fairing::AdHoc, response::content::RawHtml};

pub(crate) fn stage() -> AdHoc {
    AdHoc::on_ignite("Infinite Scroll Stage", |rocket| async {
        rocket.mount("/infinite-scroll", routes![index, page])
    })
}

#[get("/")]
pub(crate) async fn index(db: Db) -> RawHtml<String> {
    let pagination = people::Pagination::default();
    let records = db
        .run(move |conn| people::just_page(conn, &pagination))
        .await;

    let template = Layout {
        head: markup::new! {
            title { "People Infinite Scroll" }
        },
        body: Index {
            pagination: &people::Pagination::default(),
            records: &records,
        },
    };

    RawHtml(template.to_string())
}

#[get("/page?<pagination>")]
pub(crate) async fn page(db: Db, pagination: Option<people::Pagination>) -> RawHtml<String> {
    let query_pagination = pagination.unwrap_or_default();
    let render_pagination = query_pagination.clone();
    let records = db
        .run(move |conn| people::just_page(conn, &query_pagination))
        .await;

    RawHtml(
        Pager {
            pagination: &render_pagination,
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
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "First Name" } }
                    div[class="flex items-center flex-grow w-0 h-10 px-2 border-b border-l border-black"] { span { "Last Name" } }
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
