use rocket::{response::Redirect, request::Form, State};
use rocket_contrib::{database::DbConn, templates::Template};
use rocket_jwt::{Claims, JWT, Signer, Token};
use crate::auth::auth;

#[derive(FromForm)]
struct LoginForm {
    ip_address: String,
    password: String,
}

#[post("/login", data = "<form>")]
fn login(
    jwt: State<JWT<MyClaims>>,
    form: Form<LoginForm>,
    conn: DbConn,
) -> Result<Redirect, Template> {
    use schema::users::dsl::*;

    let ip_address = form.ip_address.parse::<Ipv4Addr>().map_err(|_| {
        Template::render("login", &format!("Invalid IP address: {}", form.ip_address))
    })?;

    if !is_local_ipv4_address(&ip_address) {
        return Err(Template::render(
            "login",
            &format!("IP address is not local: {}", ip_address),
        ));
    }
    let user = users
        .filter(ip_address.eq(&form.ip_address))
        .first::<User>(&*conn)
        .optional()
        .map_err(|_| Template::render("login", "Error fetching user from database"))?;

    if let Some(user) = user {
        if user.password != form.password {
            return Err(Template::render("login", "Invalid password"));
        }

        let claims = MyClaims { ip_address: user.ip_address };
        let token = jwt.sign(claims)?;

        Ok(Redirect::to(format!("/files?token={}", token)))
    } else {
        Err(Template::render("login", "User not found"))
    }
}
