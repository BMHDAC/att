use std::sync::{Arc, Mutex};

use axum::{extract::State, routing::post, Json, Router};
use chrono::NaiveDate;
use hyper::StatusCode;
use serde::Deserialize;
use tracing::error;

use crate::{
    configs::database::Users,
    shared::{
        http::{ApiResponse, ErrorResponse},
        state::AppState,
    },
};

pub fn auth_routes(state: Arc<Mutex<AppState>>) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .route("/auth/register", post(register))
        .with_state(state)
}

pub async fn login(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(user): Json<LoginRequest>,
) -> Result<Json<ApiResponse<Users>>, ErrorResponse> {
    let database = state.lock().map_err(|e| {
        error!("{}", e.to_string());
        ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error",None )
    })?.db.clone();

    let user =
        sqlx::query_as::<_, Users>(r#"select * from users where email = $1 and password = $2"#)
            .bind(user.email)
            .bind(user.password)
            .fetch_one(&database)
            .await?;
    Ok(Json(ApiResponse {
        data: user,
        meta: None,
    }))
}

pub async fn register(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(new_user): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<String>>, ErrorResponse> {
    let database = state.lock().map_err(|e|{
         error!("{}", e.to_string());
         ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error", None)
    })?.db.clone();

    let query_result = 
        sqlx::query("insert into users(id, email, password, dob, username, fullname, address, avatar_url, alias,org_name) values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)")
        .bind(new_user.id)
        .bind(new_user.email)
        .bind(new_user.password)
        .bind(new_user.dob)
        .bind(new_user.username)
        .bind(new_user.fullname)
        .bind(new_user.address)
        .bind(new_user.avatar_url)
        .bind(new_user.alias)
        .bind(new_user.org_name)
        .execute(&database).await?;

    Ok(Json(ApiResponse {
        data: format!("Create {} users successfully", query_result.rows_affected()) ,
        meta: None,
    }))
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
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
