#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::response::{self, Responder};
use rocket::Request;
use rocket_contrib::json::Json;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models;
use crate:: schema;

use self::models::{Manga, NewManga};

#[database("sqlite_database")]
struct DbConn(SqliteConnection);

#[derive(Serialize, Deserialize)]
struct MangaList {
    mangas: Vec<Manga>
}

#[get("/mangas")]
fn get_mangas(conn: DbConn) -> Result<Json<MangaList>, Custom<String>> {
    use self::schema::mangas::dsl::*;

    let results = mangas.load::<Manga>(&conn);

    match results {
        Ok(mangas) => Ok(Json(MangaList { mangas })),
        Err(_) => Err(Custom(Status::InternalServerError, String::from("Error loading mangas")))
    }
}
