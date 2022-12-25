use crate::utils;

#[put("/scan")]
fn scan(conn: DbConn, jwt: JWT<MyClaims>) -> Result<String, Custom<String>> {
    use self::schema::settings::dsl::*;
    let universal_path = settings
        .filter(key.eq("universal_path"))
        .select(value)
        .first::<String>(&conn)?;

    utils::scan_new_manga_files(&conn, &universal_path)?;

    Ok(String::from("Successfully scanned directory for new mangas"))
}