use sqlx::{SqlitePool};
use axum:: {
    Json,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    body::Body,
};
use crate::utils::scanner::scan_dir;

pub enum ScanErrors {
    SqlxError(sqlx::Error),
}

impl From<sqlx::Error> for ScanErrors {
    fn from(e: sqlx::Error) -> Self {
        Self::SqlxError(e)
    }
}

impl IntoResponse for ScanErrors {
    fn into_response(self) -> Response {
        let body = match self {
            ScanErrors::SqlxError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json("Database Error"),
            ),
        }.into_response();

        return body;
    }
}


impl From<std::io::Error> for ScanErrors {
    fn from(e: std::io::Error) -> Self {
        Self::SqlxError(sqlx::Error::Io(e))
    }
}


pub async fn scan() -> Result<(), ScanErrors>{
    scan_dir()?;
    Ok(())
}
