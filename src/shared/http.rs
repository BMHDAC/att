use axum::{response::IntoResponse, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

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

pub enum ErrorResponse {
    BadRequest,
    NotFound,
    Conflict,
    InternalError,
    Unauthorized,
    Forbidden,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorBody {
    message: String,
    status_code: u16,
}

impl ErrorBody {
    fn new(code: StatusCode, message: &str) -> Self {
        Self {
            message: message.to_string(),
            status_code: code.as_u16(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        match self {
            ErrorResponse::BadRequest => (
                StatusCode::BAD_REQUEST,
                Json(ErrorBody::new(StatusCode::BAD_REQUEST, "Bad Request")),
            )
                .into_response(),
            ErrorResponse::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorBody::new(StatusCode::NOT_FOUND, "Not Found")),
            )
                .into_response(),
            ErrorResponse::Conflict => (
                StatusCode::CONFLICT,
                Json(ErrorBody::new(StatusCode::CONFLICT, "Conflict")),
            )
                .into_response(),
            ErrorResponse::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorBody::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error",
                )),
            )
                .into_response(),
            ErrorResponse::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorBody::new(
                    StatusCode::UNAUTHORIZED,
                    "Unauthorized Access",
                )),
            )
                .into_response(),
            ErrorResponse::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorBody::new(StatusCode::FORBIDDEN, "Forbidden Resources")),
            )
                .into_response(),
        }
    }
}
