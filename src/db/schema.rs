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

diesel::table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}

use diesel::allow_tables_to_appear_in_same_query;
use super::models::Setting;

#[derive(Insertable)]
#[table_name = "settings"]
pub struct NewSetting<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

allow_tables_to_appear_in_same_query!(settings,);