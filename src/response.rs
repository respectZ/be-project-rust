use actix_web::{body::BoxBody, http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use serde_json::json;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(skip)]
    pub status_code: StatusCode,
    pub message: String,
    pub error_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OkResponse {
    pub message: String,
    pub data: Option<serde_json::Value>,
}

impl ErrorResponse {
    pub fn new(status_code: StatusCode, message: String, error_code: Option<String>) -> Self {
        ErrorResponse {
            status_code,
            message,
            error_code,
        }
    }
}

impl OkResponse {
    pub fn new(message: String, data: Option<serde_json::Value>) -> HttpResponse<BoxBody> {
        HttpResponse::build(StatusCode::OK)
            .json(json!({
                "message": message,
                "data": data,
            }))
            .into()
    }
}

impl ResponseError for ErrorResponse {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code)
            .json(json!({
                "message": self.message,
                "error_code": self.error_code,
            }))
            .into()
    }

    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let deserialized = serde_json::to_string(self).unwrap();
        write!(f, "{}", deserialized)
    }
}
