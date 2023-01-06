
use axum::{
    Json,
    http::{StatusCode, Request},
    response::{IntoResponse, Response},
    extract::{State, BodyStream},
    body,
};
use crate::db::models::{DatabaseSettings, ServerSettings};
use crate::utils::hash::hash_password;

pub enum SetPasswordErrors {
    UnableToSetPassword,
}

impl IntoResponse for SetPasswordErrors {
    fn into_response(self) -> Response {
        let(status, err_msg) = match self {
            SetPasswordErrors::UnableToSetPassword => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to Set Password",
            ),
        };
        (status, err_msg).into_response()
    }
}

#[axum_macros::debug_handler]
pub async fn set_password (body: String) -> Result<String, SetPasswordErrors> {
    //env::set_var(PASSWORD, hash_password(&body));
    Ok("Password set successfully".to_owned())
}