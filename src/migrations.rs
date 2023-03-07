use once_cell::sync::Lazy;
use rusqlite_migration::{Migrations, M};

pub(crate) static MIGRATIONS: Lazy<Migrations<'static>> = Lazy::new(|| {
    Migrations::new(vec![
        M::up(include_str!("../migrations/20220122175227_people.up.sql"))
            .down(include_str!("../migrations/20220122175227_people.down.sql")),
        M::up(include_str!("../migrations/20220427164619_seed.up.sql"))
            .down(include_str!("../migrations/20220427164619_seed.down.sql")),
    ])
});
