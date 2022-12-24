// @generated automatically by Diesel CLI.

diesel::table! {
    mangas (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
    }
}

diesel::table! {
    users (id) {
        password -> Text,
    }
}