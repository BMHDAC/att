use std::sync::{Arc, Mutex};

use axum::{extract::State, routing::post, Json, Router};
use chrono::NaiveDate;
use hyper::{header::USER_AGENT, HeaderMap, StatusCode};
use serde::Deserialize;
use tracing::error;

use crate::{
    configs::database::Users,
    shared::{
        http::{ApiResponse, ErrorResponse},
        state::{AppState, DeviceType, SessionToken},
    },
};

pub fn auth_routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .with_state(state)
}

pub async fn login(
    header: HeaderMap,
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<ApiResponse<String>>, ErrorResponse> {
    let database = state
        .lock()
        .map_err(|e| {
            error!("{}", e.to_string());
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                None,
            )
        })?
        .db
        .clone();

    let found_user = sqlx::query_as_unchecked!(
        Users,
        "select * from users where email = $1 and password = $2",
        request.email,
        request.password
    )
    .fetch_one(&database)
    .await?;

    let sessions = &mut state
        .lock()
        .map_err(|e| {
            error!("{}", e.to_string());
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                None,
            )
        })?
        .session;

    let user_agent = header
        .get(USER_AGENT)
        .unwrap()
        .to_str()
        .map_err(|e| {
            error!("{}", e.to_string());
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                Some("Invalid user-agent".to_string()),
            )
        })?
        .to_string();
    let new_session = SessionToken {
        expired_date: None,
        user_agent,
        device_id: request.device_id,
        device_type: DeviceType::Desktop,
    };
    sessions.insert(found_user.id, new_session);

    Ok(Json(ApiResponse {
        data: String::from("Success"),
        meta: None,
    }))
}

pub async fn register(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<String>>, ErrorResponse> {
    let database = state
        .lock()
        .map_err(|e| {
            error!("{}", e.to_string());
            ErrorResponse::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
                None,
            )
        })?
        .db
        .clone();

    let query_result = sqlx::query!(r#"insert into users(id, email, password, dob, username, fullname, address, avatar_url, alias, org_name) values($1, $2,$3, $4, $5, $6, $7, $8, $9, $10)"#, request.id, request.email,request.password, request.dob,request.username, request.fullname,request.address, request.avatar_url, request.alias, request.org_name).execute(&database).await?;

    if query_result.rows_affected() != 1 {
        return Err(ErrorResponse::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error",
            Some("Failed to insert into database".to_string()),
        ));
    }
    Ok(Json(ApiResponse {
        data: format!("Successfully created {} user", query_result.rows_affected()),
        meta: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    id: String,
    email: String,
    password: String,
    dob: NaiveDate,
    username: String,
    fullname: String,
    address: Option<String>,
    avatar_url: Option<String>,
    alias: Option<String>,
    org_name: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    email: String,
    password: String,
    device_id: String,
}
