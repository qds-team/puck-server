use axum::{
    Json,
    http::{StatusCode, Request},
    response::{IntoResponse, Response},
    extract::{State, BodyStream},
    body,
};

use super::set_password::SetPasswordErrors;

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
pub async fn set_path (body: String) -> Result<String, SetPasswordErrors> {
    //TODO: Set Path in env file

    Ok("Path set successfully".to_owned())
}