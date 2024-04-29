use crate::diesel_ext::dsl::*;
use crate::schema::{people, people_fts};
use diesel::prelude::*;
use regex::Regex;

#[derive(Clone, Debug, FromForm)]
pub struct Pagination {
    pub(crate) page: i64,
    pub(crate) per_page: i64,
    pub(crate) search: Option<String>,
}

pub(crate) mod model {
    use diesel::prelude::*;

    #[derive(Debug, Queryable, Selectable)]
    #[diesel(table_name = crate::schema::people)]
    #[allow(dead_code)]
    pub struct Person {
        pub(crate) rowid: i64,
        pub(crate) id: String,
        pub(crate) first_name: String,
        pub(crate) last_name: String,
    }

    #[derive(Debug, Queryable)]
    pub struct SearchResult {
        pub(crate) id: String,
        pub(crate) name: String,
    }
}

impl Pagination {
    fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }

    pub(crate) fn next_page(&self, records: &[model::Person]) -> Option<i64> {
        if (records.len() as i64) == self.per_page {
            Some(self.page + 1)
        } else {
            None
        }
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page: 1,
            per_page: 100,
            search: None,
        }
    }
}

pub(crate) fn perform_search(
    connection: &mut SqliteConnection,
    pagination: &Pagination,
) -> Vec<model::SearchResult> {
    let search = pagination.search.as_ref().unwrap();
    let wildcard = format!("{search}*");

    let mut results: Vec<model::SearchResult> = people_fts::table
        .select((
            people_fts::id,
            people_fts::last_name
                .concat(", ")
                .concat(people_fts::first_name),
        ))
        .filter(
            people_fts::last_name
                .matches(&wildcard)
                .or(people_fts::first_name.matches(&wildcard)),
        )
        .order((people_fts::last_name, people_fts::first_name))
        .limit(pagination.per_page)
        .offset(pagination.offset())
        .get_results(connection)
        .unwrap();

    let format = format!("((?i){search})");
    let regex = Regex::new(format.as_str()).unwrap();
    for result in &mut results {
        result.name = regex
            .replace_all(&result.name, "<mark>$1</mark>")
            .to_string();
    }

    results
}

pub(crate) fn just_page(
    connection: &mut SqliteConnection,
    pagination: &Pagination,
) -> Vec<model::Person> {
    let (people1, people2) = diesel::alias!(people as people1, people as people2);

    // Performance is much faster than doing pure
    //   "select * from <table> where <clause> limit <x> offset <y>"
    // See https://stackoverflow.com/a/49651023
    people1
        .filter(
            people1.field(people::rowid).eq_any(
                people2
                    .select(people2.field(people::rowid))
                    .order(people2.fields((people::last_name, people::first_name)))
                    .limit(pagination.per_page)
                    .offset(pagination.offset()),
            ),
        )
        .order(people1.fields((people::last_name, people::first_name)))
        .get_results(connection)
        .unwrap()
}
