use axum::{
    routing::get,
    Router,
};
//use serde::{Deserialize, Serialize};
use dotenv::dotenv;
use std::net::SocketAddr;
use crate::routes::get_media::{get_media, get_media_file};

//import local modules
mod routes; 
mod db;
mod auth;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    //create db
    /* 
    dotenv().ok();
    db::create_db("DATABSE_URL");
    */
    // build our application with a route
    let app = Router::new()
        .route("/media/:id", get(get_media))
        .route("/media/:id/:filename", get(get_media_file));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
