use axum::{
    routing::{get,post, put},
    Router,
    extract::State,
};
use crate::routes::{
    get_media::{get_media, get_media_file},
    login::login,
    set_password::set_password,
};
use std::net::SocketAddr;
use sqlx::SqlitePool;
use utils::env_load::get_db_url;

//import local modules
mod routes; 
mod db;
mod auth;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    //create db connection
    let pool = SqlitePool::connect(&get_db_url()).await?;

    // build our application with a route
    let app = Router::new()
        .route("/media/:id", get(get_media))
        .route("/media/:id/:filename", get(get_media_file))
        .route("/login", post(login))
        .route("/set-password", put(set_password))
        .with_state(pool); //passing db connection to all 

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service()) 
        .await
        .unwrap();
    
    Ok(())
}
