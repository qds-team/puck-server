use std::path::{Path, PathBuf};
use rocket::{Rocket, State};
use rocket_contrib::json::Json;
use rocket_jwt::{JWT, RocketJWT};

#[get("/manga/<id>")]
fn get_manga(id: i32, jwt: JWT<MyClaims>) -> Result<Json<Vec<String>>, DieselError> {
    use diesel::prelude::*;
    use schema::manga_files::dsl::*;

    let connection = establish_connection();
    let files = manga_files
        .filter(manga_id.eq(id))
        .load::<MangaFile>(&connection)?
        .into_iter()
        .map(|file| file.name)
        .collect::<Vec<String>>();

    Ok(Json(files))
}

#[get("/manga/<id>/<filename>")]
fn get_manga_file(id: i32, filename: String, jwt: JWT<MyClaims>) -> Result<NamedFile, DieselError> {
    use diesel::prelude::*;
    use schema::manga_files::dsl::*;

    let connection = establish_connection();

    let manga_file = manga_files
        .filter(manga_id.eq(id))
        .filter(filename.eq(&filename))
        .first::<MangaFile>(&connection)?;

    Ok(NamedFile::open(manga_file.path)?)
}
