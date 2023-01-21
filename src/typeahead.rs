use crate::people::{perform_search, Pagination, Person};

use askama::Template;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
};
use serde::Deserialize;
use sqlx::SqlitePool;

pub(crate) async fn index() -> impl IntoResponse {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "typeahead/index.html")]
struct IndexTemplate {}

pub(crate) async fn results(
    query: Query<Submission>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    let Query(submission) = query;
    let pagination = Pagination {
        page: 1,
        per_page: 10,
        search: Some(submission.query.clone()),
    };
    let records = perform_search(&pool, &pagination).await;
    ResultsTemplate {
        submission,
        records,
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
    records: Vec<Person>,
}

mod filters {
    use crate::people::Person;
    use regex::Regex;

    pub(crate) fn highlight(s: &str, search: &String) -> askama::Result<String> {
        let regex = format!("(?i)(?P<find>{search})");
        let re = Regex::new(regex.as_str()).unwrap();
        let result = re.replace_all(s, "<mark>$find</mark>".to_string());
        Ok(result.to_string())
    }

    pub(crate) fn format_person(person: &Person) -> askama::Result<String> {
        let first_name = person.first_name.clone();
        let last_name = person.last_name.clone();
        let result = format!("{last_name}, {first_name}");
        Ok(result)
    }
}
