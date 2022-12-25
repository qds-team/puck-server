// schema.rs

diesel::table! {
    mangas (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
    }
}

diesel::table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        ip -> Text,
        password_hash -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(mangas, settings, users,);
