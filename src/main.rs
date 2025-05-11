#![warn(clippy::all)]
#![allow(clippy::blocks_in_conditions)] // Until https://github.com/rwf2/Rocket/issues/2655 is released
#![allow(clippy::needless_lifetimes)] // Until clippy is fixed https://github.com/rust-lang/rust-clippy/issues/13811
#![deny(unreachable_pub, private_bounds, private_interfaces)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate rocket;

mod assets;
mod diesel_ext;
pub mod layout;
pub mod people;
mod schema;
pub mod scroll;
pub mod typeahead;

use diesel::sqlite::SqliteConnection;
use layout::Layout;
use rocket::{fairing::AdHoc, response::content::RawHtml, Build, Rocket};
use rocket_sync_db_pools::database;

#[database("demo")]
struct Db(SqliteConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("Diesel SQLite Stage", |rocket| async {
            rocket
                .attach(Db::fairing())
                .attach(AdHoc::on_ignite("Diesel Migrations", run_migrations))
        }))
        .mount("/", routes![directory])
        .attach(scroll::stage())
        .attach(typeahead::stage())
        .attach(assets::stage())
}

async fn run_migrations(rocket: Rocket<Build>) -> Rocket<Build> {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

    const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

    Db::get_one(&rocket)
        .await
        .expect("failure obtaining database connection")
        .run(|conn| {
            conn.run_pending_migrations(MIGRATIONS)
                .expect("failure running diesel migrations");
        })
        .await;

    rocket
}

#[get("/")]
async fn directory() -> RawHtml<String> {
    let template = Layout {
        head: markup::new! {
            title { "Demo Directory" }
        },
        body: markup::new! {
            main {
              ul {
                li { a[href="/typeahead-search"] { "Typeahead Searching" } }
                li { a[href="/infinite-scroll"] { "Infinite Scroll" } }
              }
            }
        },
    };

    RawHtml(template.to_string())
}
