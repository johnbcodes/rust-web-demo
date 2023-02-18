use serde::Deserialize;
use sqlx::{query_as, FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub(crate) struct Person {
    pub(crate) id: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct Pagination {
    pub(crate) page: i64,
    pub(crate) per_page: i64,
    pub(crate) search: Option<String>,
}

impl Pagination {
    fn offset(&self) -> i64 {
        (self.page - 1) * self.per_page
    }

    pub(crate) fn next_page(&self, records: &Vec<Person>) -> Option<i64> {
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
            per_page: 30,
            search: None,
        }
    }
}

pub(crate) async fn load(pool: &SqlitePool, pagination: &Pagination) -> Vec<Person> {
    if pagination.search.is_some() {
        perform_search(pool, pagination).await
    } else {
        just_page(pool, pagination).await
    }
}

pub(crate) async fn perform_search(pool: &SqlitePool, pagination: &Pagination) -> Vec<Person> {
    let fmt = pagination.search.as_ref().unwrap();
    let search = format!("{fmt}*");
    // language=SQL
    let sql = r#"
      select
        *
      from people_fts
      where (first_name match ?1 or last_name match ?1)
      order by last_name, first_name
      limit ?2
      offset ?3
    "#;
    query_as(sql)
        .bind(search)
        .bind(pagination.per_page)
        .bind(pagination.offset())
        .fetch_all(pool)
        .await
        .unwrap()
}

async fn just_page(pool: &SqlitePool, pagination: &Pagination) -> Vec<Person> {
    // language=SQL
    let sql = r#"
      select
        *
      from people
      where rowid in (
        select
          rowid
        from people
        order by last_name, first_name
        limit ?1
        offset ?2)
      order by last_name, first_name
    "#;
    query_as(sql)
        .bind(pagination.per_page)
        .bind(pagination.offset())
        .fetch_all(pool)
        .await
        .unwrap()
}
