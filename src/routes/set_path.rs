use std::path::PathBuf;
use rocket::{Rocket, State};
use rocket_jwt::{JWT, RocketJWT};

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use crate::models::Setting;
use crate::schema::settings::dsl::*;

#[database("sqlite_database")]
struct DbConn(SqliteConnection);

#[post("/path", data = "<path>")]
fn set_path(conn: DbConn, path: PathBuf, jwt: JWT<MyClaims>) -> Result<String, Custom<String>> {
    let path = path.to_str().unwrap();

    diesel::update(settings.filter(key.eq("universal_path")))
        .set(value.eq(path))
        .execute(&conn)?;

    Ok(String::from("Successfully set universal path"))
}
