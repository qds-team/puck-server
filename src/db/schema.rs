// schema.rs

diesel::table! {
    manga (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
        files -> Array<Text>,
    }
}

diesel::table! {
    manga_files (id) {
        id -> Integer,
        manga_id -> Integer,
        filename -> Text,
    }
}

diesel::joinable!(manga_files -> manga (manga_id));

diesel::allow_tables_to_appear_in_same_query!(manga, manga_files);

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

diesel::allow_tables_to_appear_in_same_query!(manga, settings, users, manga_files);
