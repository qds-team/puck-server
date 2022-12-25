// @generated automatically by Diesel CLI.

diesel::table! {
    mangas (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
    }
}

#[derive(Insertable)]
#[table_name = "mangas"]
struct NewManga<'a> {
    name: &'a str,
    path: &'a str,
}

dissel::table! {
    users (ip_address) {
        ip_address -> Text,
        password -> Text,
    }
}

#[derive(Insertable)]
#[table_name = "users"]
struct NewUser<'a> {
    ip_address: &'a str,
    password: &'a str,
}