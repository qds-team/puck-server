#[macro_use] extern crate rocket;
use rocket::fs::FileServer;

#[launch]
fn rocket() -> _ {
    //rocket::build().mount("/public", FileServer::from("/static"))
}

#[get("/setPath")]
fn set_file_path(){
    /*send list of comic links*/
}

#[get("/receiveComicName")]
fn res_comic_name(){
    /*serve comic file*/
}
