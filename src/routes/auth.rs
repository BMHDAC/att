use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use hyper::Response;
use serde::Deserialize;
use sqlx::query_as;

use crate::{
    configs::database::Users,
    shared::{
        http::{ApiResponse, ErrorResponse},
        state::AppState,
    },
};

pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/auth/login", post(login))
        .with_state(state)
}

pub async fn login(
    State(state): State<AppState>,
    Json(user): Json<LoginRequest>,
) -> Result<Json<ApiResponse<Users>>, ErrorResponse> {
    let database = &state.db;

    let user = query_as!(
        Users,
        r#"select * from users where email = $1 and password = $2"#,
        user.email,
        user.password
    )
    .fetch_one(database)
    .await;

    if let Ok(user) = user {
        return Ok(Json(ApiResponse {
            data: user,
            meta: None,
        }));
    }

    Err(ErrorResponse::Forbidden)
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}
