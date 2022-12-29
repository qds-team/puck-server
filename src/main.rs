extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_jwt;
extern crate tokio;


use std::thread::spawn;
use std::time::Duration;
use rocket::routes;
use tokio::time::delay_for;

mod db; // import the db module
mod utils; // import the utils module
mod routes;  //import routes module

#[tokio::main]
async fn main() {
    // Start the scan task
    let conn = db::establish_connection(); // establish the connection here
    let task = async move {
        loop {
            // Scan the directory
            utils::scan_new_manga_files(&conn, &universal_path).unwrap(); // pass the connection to scan_new_manga_files

            // Wait for 5 minutes before scanning again
            delay_for(Duration::from_secs(300)).await;
        }
    };
    spawn(task);

    // Mount the routes
    rocket::ignite()
        .manage(conn) // pass the connection to Rocket
        .mount("/api", routes![login, register, scan, set_path, get_mangas, get_manga, get_manga_file])
        .launch();
}
