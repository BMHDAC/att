use std::error::Error;

use configs::database::create_pool;
use routes::public::public_routes;
use tracing::info;

mod configs;
mod routes;
mod shared;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt().init();
    let addr = tokio::net::TcpListener::bind("127.0.0.1:8888").await?;
    let pg_database = create_pool("localhost", "thomas", "123456", "att").await?;
    info!("Starting server on port 8888");
    axum::serve(addr, public_routes()).await?;
    Ok(())
}
