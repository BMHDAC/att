use std::error::Error;

use configs::database::create_pool;
use routes::{auth::auth_routes, public::public_routes};
use shared::state::AppState;
use tracing::info;

mod configs;
mod routes;
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let addr = tokio::net::TcpListener::bind("127.0.0.1:8888").await?;
    let pg_database = create_pool("localhost", "thomas", "123456", "att").await?;
    let state = AppState { db: pg_database };
    info!("Starting server on port 8888");
    axum::serve(addr, public_routes().merge(auth_routes(state))).await?;
    Ok(())
}
