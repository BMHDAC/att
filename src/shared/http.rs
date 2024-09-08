use axum::response::IntoResponse;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: Option<ResponseMeta>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseMeta {
    page_size: u8,
    page: u8,
    item_count: u8,
}

#[derive(Debug)]
pub struct ErrorResponse {
    message: String,
    status_code: StatusCode,
    cause: Option<String>,
}

impl ErrorResponse {
    pub fn new(code: StatusCode, message: &str, cause: Option<String>) -> Self {
        Self {
            message: message.to_string(),
            status_code: code,
            cause,
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        let payload = json!({
            "message": self.message,
            "statusCode": self.status_code.as_u16(),
            "cause": self.cause
        });

        (self.status_code, axum::Json(payload)).into_response()
    }
}

impl From<sqlx::Error> for ErrorResponse {
    fn from(value: sqlx::Error) -> Self {
        error!("{}", value.to_string());
        match value {
            sqlx::Error::RowNotFound => Self::new(StatusCode::NOT_FOUND, "Not found", None),
            sqlx::Error::TypeNotFound { type_name } => Self::new(
                StatusCode::NOT_FOUND,
                "Not Found",
                Some(format!("Type: {type_name} not found")),
            ),
            sqlx::Error::ColumnNotFound(e) => {
                Self::new(StatusCode::NOT_FOUND, "Not Found", Some(e.to_string()))
            }
            sqlx::Error::Database(e) => {
                Self::new(StatusCode::CONFLICT, "Database Error", Some(e.to_string()))
            }
            _ => Self::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Interal server Error",
                None,
            ),
        }
    }
}
