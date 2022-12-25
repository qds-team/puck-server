use std::path::PathBuf;
use rocket::{Rocket, State};
use rocket_jwt::{JWT, RocketJWT};

#[get("/manga/<id>")]
fn get_manga(id: i32, jwt: JWT<MyClaims>) -> Result<NamedFile, DieselError> {
    use diesel::prelude::*;
    use schema::manga::dsl::*;

    let connection = establish_connection();
    let manga = manga.find(id).first::<Manga>(&connection)?;

    Ok(NamedFile::open(manga.path)?)
}
