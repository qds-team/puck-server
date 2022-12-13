#[macro_use] extern crate rocket;
use rocket::{get, post, form::Form, routes};
use rocket_auth::{Users, Error, Auth, Signup, Login};

#[launch]
fn rocket() -> _ {

}

#[put("/set-path")]
fn set_file_path() {
    //TODO: Check if Client is Authenticated
    //TODO: Set Path of Mangas
}

#[get("/get-media-list")]
fn get_media_list() {
    //TODO: Check if Client is Authenticated
    //TODO: Return JSON list of Manga Name, and Manga ID's
}

#[get("/get-media")]
fn get_media(manga_id: &str) {
    //TODO: Check if Client is Authenticated
    //TODO: Implement file server to serve manga file
}
