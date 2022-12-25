use rocket::{post, request::Form, State};
use rocket_contrib::database;
use rocket_jwt::{JWT, RocketJWT};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(FromForm)]
struct RegisterForm {
    ip_address: String,
    password: String,
}

#[post("/register", data = "<form>")]
fn register(conn: DbConn, form: Form<RegisterForm>, jwt: JWT<MyClaims>) -> Result<String, diesel::result::Error> {
    use schema::users::dsl::*;

    let new_user = NewUser {
        ip_address: &form.ip_address,
        password: &form.password,
    };

    diesel::insert_into(users)
        .values(&new_user)
        .execute(&conn)?;

    Ok("User registered successfully".to_string())
}