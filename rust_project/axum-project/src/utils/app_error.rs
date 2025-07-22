use axum::{Json, http::StatusCode, response::IntoResponse};

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new<S>(code: StatusCode, message: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.code, Json(self.message.clone())).into_response()
    }
}
