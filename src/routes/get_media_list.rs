use axum::response::Response;

use crate::db::models::{Manga, MangaFiles};


pub enum GetMediaListErrors {
    SqlxError(sqlx::Error),
    NoMedia,
}

impl From<sqlx::Error> for GetMediaListErrors {
    fn from(e: sqlx::Error) -> Self {
        Self::SqlxError(e)
    }
}

impl IntoResponse for GetMediaListErrors {
    fn into_response(self) -> Response {
        let body = match self {
            GetMediaListErrors::SqlxError(e) => (
                Status::INTERNAL_SERVER_ERROR,
                Json("Database Error"),
            ),
            GetMediaListErrors::NoMedia(e) => (
                Status::INTERNAL_SERVER_ERROR,
                Json("Database Error"),
            ),
        }.into_response();

        return body;
    }
}

pub async fn get_media_list(State(pool): State<SqlitePool>) -> Result<Json<Vec<Manga>>, GetMediaListErrors> {
    //TODO:: Implement SQL Query & Add Authentication

    let media_list: Vec<Manga> = sqlx::query_as(
        "",
    )
    .fetch_all(&mut pool.accquire().await?)
    .await?;

    Ok(Json(media_list))
}