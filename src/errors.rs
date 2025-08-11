use axum::{http::StatusCode, response::IntoResponse};

pub struct GlobalAppError {
    error_code: StatusCode,
    message: String,
}

impl IntoResponse for GlobalAppError {
    fn into_response(self) -> axum::response::Response {
        (self.error_code, self.message).into_response()
    }
}

impl GlobalAppError {
    pub fn new(error_code: StatusCode, message: String) -> Self {
        Self {
            error_code,
            message,
        }
    }
}
