use fake::Fake;
use fake::faker::name::raw::{FirstName, LastName};
use fake::locales::EN;
use nanoid::nanoid;
use std::fs::File;
use std::io::prelude::*;
use std::io::LineWriter;

fn main() -> std::io::Result<()> {
    let file = File::create("seed.sql")?;
    let mut file = LineWriter::new(file);
    for _n in 1..=10_000 {
        let id = nanoid!();
        let first_name: String = str::replace(FirstName(EN).fake(), "'","''");
        let last_name: String = str::replace(LastName(EN).fake(), "'","''");
        file.write_all(format!("insert into people (id, first_name, last_name) values ('{id}','{first_name}','{last_name}');\n").as_ref())?;
    }
    file.write_all("reindex people_first_name_trgm_idx;\n".as_ref())?;
    file.write_all("reindex people_last_name_trgm_idx;\n".as_ref())?;
    Ok(())
}