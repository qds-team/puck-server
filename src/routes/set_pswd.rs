use rocket::{post, request::Form, State};
use rocket_contrib::database;
use rocket_jwt::{JWT, RocketJWT};

#[database("sqlite_database")]
pub struct DbConn(diesel::SqliteConnection);

#[derive(FromForm)]
struct SetPasswordForm {
    ip_address: String,
    password: String,
}

#[post("/set-password", data = "<form>")]
fn set_password(conn: DbConn, form: Form<SetPasswordForm>, jwt: JWT<MyClaims>) -> Result<String, diesel::result::Error> {
    use schema::users::dsl::*;

    diesel::update(users.filter(ip_address.eq(form.ip_address)))
        .set(password.eq(form.password))
        .execute(&conn)?;

    Ok("Password updated successfully".to_string())
}
