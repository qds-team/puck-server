
use sqlx::{SqlitePool};
use crate::db::models::{MangaFiles, Manga};
use axum::{
    Json,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    body::{BoxBody, boxed, Body},
    extract::Path,
};
use dotenv::dotenv;
use tower::ServiceExt;
use tower_http::services::ServeDir;
use serde::Deserialize;

pub enum GetMediaErrors {
    SqlxError(sqlx::Error),
    MediaDoesNotExist,
    FileDoesNotExist,
}

impl From<sqlx::Error> for GetMediaErrors{
    fn from(e: sqlx::Error) -> Self {
        Self::SqlxError(e)
    }
}

impl IntoResponse for GetMediaErrors {
    fn into_response(self) -> Response {
        let body  = match self {
            GetMediaErrors::MediaDoesNotExist => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Media you requested does not exist on server."),
            ),
            GetMediaErrors::FileDoesNotExist => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("File for media you requested does not exist on server."),
            ),
            GetMediaErrors::SqlxError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Database Error"),
            ),
        }.into_response();

        return body;
    }
}

#[axum_macros::debug_handler]
pub async fn get_media(Path(id): Path<i32>) -> Result<Json<Vec<MangaFiles>>, GetMediaErrors> {
    dotenv().ok();
    let pool = SqlitePool::connect("DATABASE_URL").await?;

    let manga_files: Vec<MangaFiles> = sqlx::query_as!(
        MangaFiles,
        "SELECT * FROM manga_files WHERE manga_id = $1",
        id
    )
    .fetch_all(&mut pool.acquire().await?)
    .await?;

    //TODO:  add if statement to check for sqlx error then throw more specific error

    Ok(Json(manga_files))
}

#[derive(Deserialize)]
pub struct Params {
    id: i32,
    filename: String,
}

#[axum_macros::debug_handler]
pub async fn get_media_file( 
    Path(Params {id, filename}): Path<Params>,
) -> Result<Response<BoxBody>, GetMediaErrors> {
    dotenv().ok();
    let pool = SqlitePool::connect("DATABASE_URL").await?;

    let manga = sqlx::query_as!(
        Manga,
        "SELECT * FROM manga WHERE id = $1", id,
    )
    .fetch_one(&mut pool.acquire().await?)
    .await?;

    let manga_file: MangaFiles = sqlx::query_as!(
        MangaFiles,
        "SELECT * FROM manga_files WHERE manga_id = $1 AND filename = $2", id, filename,
    )
    .fetch_one(&mut pool.acquire().await?)
    .await?;

    //TODO:  add if statement to check for sqlx error then throw more specific error

    let path: String = manga.path + "/"+ &manga_file.filename;
    let res = get_static_manga(path).await?;

    Ok(res)
}

async fn get_static_manga(path: String) -> Result<Response<BoxBody>, GetMediaErrors> {
    let req = Request::builder().body(Body::empty()).unwrap();


    match ServeDir::new(path).oneshot(req).await {
        Ok(res) => Ok(res.map(boxed)),
        Err(_err) => Err(GetMediaErrors::FileDoesNotExist),
    }
}
