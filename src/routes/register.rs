use rocket::{post, request::Form, FromForm};
use rocket_contrib::database;
use rocket_jwt::jwt;
use crate::db::models::{NewUser, User};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(FromForm)]
struct RegisterForm {
    ip_address: String,
    password: String,
}

#[post("/register", data = "<form>")]
fn register(conn: DbConn, form: Form<RegisterForm>, jwt: jwt<MyClaims>) -> Result<String, diesel::result::Error> {
    let new_user = NewUser {
        ip: &form.ip_address,
        password_hash: &form.password, //TODO: Hash password
    };

    User::register(&conn, &new_user.ip, &new_user.password)?;

    Ok("User registered successfully".to_string())
}

