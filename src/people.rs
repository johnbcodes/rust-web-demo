use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Row;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub(crate) page: i64,
    pub(crate) per_page: i64,
    pub(crate) search: Option<String>,
}

#[derive(Debug)]
pub struct Person {
    pub(crate) id: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}

#[derive(Debug)]
pub struct SearchResult {
    pub(crate) id: String,
    pub(crate) name: String,
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
            per_page: 100,
            search: None,
        }
    }
}

pub(crate) async fn perform_search(
    pool: &Pool<SqliteConnectionManager>,
    pagination: &Pagination,
) -> Vec<SearchResult> {
    let fmt = pagination.search.as_ref().unwrap();
    let search = format!("{fmt}*");
    // language=SQL
    let sql = r#"
      select
        id,
        replace(last_name || ',' || first_name, ?1, '<mark>' || ?1 || '</mark>')
      from people_fts
      where (first_name match ?1 or last_name match ?1)
      order by last_name, first_name
      limit ?2
      offset ?3
    "#;
    let connection = pool.get().unwrap();
    let mut statement = connection.prepare_cached(sql).unwrap();
    statement
        .query_map(
            (search, pagination.per_page, pagination.offset()),
            map_search_result,
        )
        .unwrap()
        .map(|result| result.unwrap())
        .collect()
}

pub(crate) async fn just_page(
    pool: &Pool<SqliteConnectionManager>,
    pagination: &Pagination,
) -> Vec<Person> {
    // Performance is faster than doing pure select * where limit offset
    // See https://stackoverflow.com/a/49651023
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
    let connection = pool.get().unwrap();
    let mut statement = connection.prepare_cached(sql).unwrap();
    statement
        .query_map((pagination.per_page, pagination.offset()), map_person)
        .unwrap()
        .map(|result| result.unwrap())
        .collect()
}

#[inline]
fn map_person(row: &Row<'_>) -> rusqlite::Result<Person> {
    Ok(Person {
        id: row.get(0)?,
        first_name: row.get(1)?,
        last_name: row.get(2)?,
    })
}

#[inline]
fn map_search_result(row: &Row<'_>) -> rusqlite::Result<SearchResult> {
    Ok(SearchResult {
        id: row.get(0)?,
        name: row.get(1)?,
    })
}
