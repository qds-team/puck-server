use std::thread::spawn;
use std::time::Duration;
use diesel::SqliteConnection;
use rocket::{Rocket, routes, State};
use rocket_jwt::RocketJWT;

mod db; // import the db module

#[tokio::main]
async fn main() {
    // Start the scan task
    let conn = db::establish_connection(); // establish the connection here
    let task = async move {
        loop {
            // Scan the directory
            db::scan_directory(&conn, &universal_path).unwrap(); // pass the connection to scan_directory

            // Wait for 5 minutes before scanning again
            delay_for(Duration::from_secs(300)).await;
        }
    };
    spawn(task);

    // Mount the routes
    rocket::ignite()
        .manage(conn) // pass the connection to Rocket
        .manage(RocketJWT::fairing())
        .mount("/api", routes![login, register, scan, set_path, get_mangas, get_manga])
        .launch();
}
