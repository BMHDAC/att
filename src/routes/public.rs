use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

pub fn public_routes() -> Router {
    Router::new().route("/", get(main_route))
}

pub async fn main_route(Query(query): Query<InfoQuery>) -> impl IntoResponse {
    Html(format!("<h1> Name: {}.\nAge: {}", query.name, query.age))
}

#[derive(Debug, Deserialize)]
pub struct InfoQuery {
    name: String,
    age: u8,
}
