// @generated automatically by Diesel CLI.

diesel::table! {
    people (id) {
        rowid -> BigInt,
        id -> Text,
        first_name -> Text,
        last_name -> Text,
    }
}

diesel::table! {
    people_fts (rowid) {
        rowid -> Integer,
        id -> Text,
        first_name -> Text,
        last_name -> Text,
    }
}
