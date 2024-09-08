use std::{
    collections::HashMap,
    error::Error,
    sync::{Arc, Mutex},
};

use configs::{database::create_pool, environment::ApplicationConfig};
use routes::{auth::auth_routes, public::public_routes};
use shared::state::AppState;
use tracing::info;

mod configs;
mod routes;
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let config = ApplicationConfig::build()?;
    let pg_database = create_pool(
        &config.db_host,
        &config.db_user,
        &config.db_password,
        &config.db_entry,
    )
    .await?;
    let addr = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port)).await?;
    let state = Arc::new(Mutex::new(AppState {
        db: pg_database,
        session: HashMap::default(),
    }));
    info!("Starting server on port {}", config.port);
    axum::serve(addr, public_routes().merge(auth_routes(state))).await?;
    Ok(())
}
