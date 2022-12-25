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
    let new_user = NewUser {
        ip: &form.ip_address,
        password: &form.password,
    };

    User::register(&conn, &new_user.ip, &new_user.password)?;

    Ok("User registered successfully".to_string())
}
