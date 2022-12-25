use rocket::{Rocket, State};
use rocket_jwt::RocketJWT;

#[tokio::main]
async fn main() {
    // Start the scan task
    let task = async move {
        loop {
            // Scan the directory
            scan_directory(&conn, &universal_path).unwrap();

            // Wait for 5 minutes before scanning again
            delay_for(Duration::from_secs(300)).await;
        }
    };
    spawn(task);

    // Mount the routes
    rocket::ignite()
        .manage(DbConn(SqliteConnection::establish("database.sqlite3").unwrap()))
        .manage(RocketJWT::fairing())
        .mount("/api", routes![login, register, scan, set_path, get_mangas, get_manga])
        .launch();
}
