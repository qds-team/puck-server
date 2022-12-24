#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::{get, post, form::Form, routes};

pub mod models;
pub mod schema;
pub mod utils/ip_utils;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::
            routes::
            routes::
        ])

}
