use axum::{
    Json,
    http::{StatusCode, Request},
    response::{IntoResponse, Response},
    extract::{State, BodyStream, Path},
    body,
};

use super::set_password::SetPasswordErrors;
use crate::utils::env_load::set_env_path;

pub enum SetPathErrors {
    UnableToSetPath,
}

impl IntoResponse for SetPathErrors {
    fn into_response(self) -> Response {
        let(status, err_msg) = match self {
            SetPathErrors::UnableToSetPath => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to Set Path",
            ),
        };
        (status, err_msg).into_response()
    }
}


#[axum_macros::debug_handler]
pub async fn set_path (Path(path): Path<String>) -> Result<String, SetPathErrors> {
    match set_env_path(path) {
        Ok(_) => Ok("Path set successfully".to_owned()),
        Err(e) => {
            println!("Error setting path: {}", e);
            Err(SetPathErrors::UnableToSetPath)
        }
    }
}