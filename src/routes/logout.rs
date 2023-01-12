use axum::response::Response;

pub enum LogoutErrors {
    LogoutError,
}

impl IntoResponse for LogoutError {
    fn into_response(self) -> Response {
        let(status, err_mesg) = match self {
            LogoutErrors::LogoutError => (
                StatusCode::UNAUTHORIZED,
                "There was a problem logging out",
            ),
        };

        (status, err_mesg).into_response()
    }
}

pub async fn logout() -> Result<String, LogoutErrors> {
    //TODO:: Implement logout route
}