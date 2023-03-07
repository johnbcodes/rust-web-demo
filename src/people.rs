use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Row;
use serde::Deserialize;

#[derive(Debug)]
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
            per_page: 100,
            search: None,
        }
    }
}

pub(crate) async fn load(
    pool: &Pool<SqliteConnectionManager>,
    pagination: &Pagination,
) -> Vec<Person> {
    if pagination.search.is_some() {
        perform_search(pool, pagination).await
    } else {
        just_page(pool, pagination).await
    }
}

pub(crate) async fn perform_search(
    pool: &Pool<SqliteConnectionManager>,
    pagination: &Pagination,
) -> Vec<Person> {
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
    let connection = pool.get().unwrap();
    let mut statement = connection.prepare_cached(sql).unwrap();
    statement
        .query_map(
            (search, pagination.per_page, pagination.offset()),
            map_record,
        )
        .unwrap()
        .map(|result| result.unwrap())
        .collect()
}

async fn just_page(pool: &Pool<SqliteConnectionManager>, pagination: &Pagination) -> Vec<Person> {
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
        .query_map((pagination.per_page, pagination.offset()), map_record)
        .unwrap()
        .map(|result| result.unwrap())
        .collect()
}

#[inline]
fn map_record(row: &Row<'_>) -> rusqlite::Result<Person> {
    Ok(Person {
        id: row.get(0)?,
        first_name: row.get(1)?,
        last_name: row.get(2)?,
    })
}
