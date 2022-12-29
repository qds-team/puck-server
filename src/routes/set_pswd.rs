use rocket::{post, request::Form, State};
use rocket_contrib::database;
use rocket_jwt::jwt;

use argon2::{self, Config};
use rand::Rng;
k
pub struct DbConn(diesel::SqliteConnection);

#[derive(FromForm)]
struct SetPasswordForm {
    ip_address: String,
    password: String,
}

#[post("/set-password", data = "<form>")]
fn set_password(conn: DbConn, form: Form<SetPasswordForm>, jwt: JWT<MyClaims>) -> Result<String, diesel::result::Error> {
    use schema::users::dsl::*;

    let hashed_password = hash_password(&form.password)?;

    diesel::update(users.filter(ip_address.eq(form.ip_address)))
        .set(password.eq(hashed_password))
        .execute(&conn)?;

    Ok("Password updated successfully".to_string())
}

fn hash_password(password: &str) -> String {
    let salt: [u8; 32] = rand::thread_rng().gen();
    let config = Config::default();
    argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap()
}
