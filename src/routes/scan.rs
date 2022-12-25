use crate::scan;

#[put("/scan")]
fn scan(conn: DbConn, jwt: JWT<MyClaims>) -> Result<String, Custom<String>> {
    use self::schema::settings::dsl::*;

    let universal_path = settings
        .filter(key.eq("universal_path"))
        .select(value)
        .first::<String>(&conn)?;

    scan::scan_directory(&conn, &universal_path)?;

    Ok(String::from("Successfully scanned directory for new mangas"))
}
