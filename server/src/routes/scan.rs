//TODO: Implement Scanner API to call check for new Media/Files
use sqlx::{SqlitePool};
use axum:: {
    Json,
    http::{Request, StatusCode},
    body::Body,
}
use crate::utils::scanner;

pub enum ScanErrors {
    SqlxError(sqlx:Error),
}

impl From<sqlx::Error> for ScanErrors {
    fn from(e: sqlx::Error) -> Self {
        Self::Sqlx(e)
    }
}

impl IntoResponse for ScanErrors {
    fn into_response(self) -> Response {
        let body = match self {
            ScanErrors::SqlxError(e) => {
                StatusCode::INTERNAL_SERVOR_ERROR,
                Json("Database Error"),
            }
        }.into_response();

        return body;
    }
}

pub async fn scan() -> Result< , ScanErrors> {
    scan_dir();
}
