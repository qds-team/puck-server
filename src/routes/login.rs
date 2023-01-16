use axum::{
    Json,
    http::{StatusCode, Request},
    response::{IntoResponse, Response},
    extract::{State, BodyStream}, body,
};
use sqlx::SqlitePool;
use crate::utils::{
    hash::verify_password,
    env_load::get_password
};


pub enum LoginErrors {
    WrongPassword,
}

impl IntoResponse for LoginErrors {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            LoginErrors::WrongPassword => (
                StatusCode::UNAUTHORIZED, 
                "Incorrect Password",
            ),
        };

        (status, err_msg).into_response()
    }
}

pub async fn login(body: String) -> Result<String, LoginErrors> {
   if verify_password(&get_password(), &body) {
        Ok("Success".to_owned())
   }
   else {
       return Err(LoginErrors::WrongPassword);
   }
}