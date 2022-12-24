#[macro_use] extern crate rocket;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket::{get, post, form::Form, routes};

fn main() {
    rocket::ignite()
        .mount("/", routes![
            routes::
            routes::
            routes::
        ])

}
