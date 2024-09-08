use std::{env, error::Error};

use tracing::info;

#[derive(Debug)]
pub struct ApplicationConfig {
    pub db_user: String,
    pub db_password: String,
    pub db_host: String,
    pub db_entry: String,
}

impl ApplicationConfig {
    pub fn build() -> Result<Self, Box<dyn Error>> {
        let db_user = env::var("DB_USER")?;
        let db_password = env::var("DB_PASSWORD")?;
        let db_host = env::var("DB_HOST")?;
        let db_entry = env::var("DB_ENTRY")?;

        info!("Environment Variables Loaded");

        Ok(Self {
            db_user,
            db_password,
            db_host,
            db_entry,
        })
    }
}
