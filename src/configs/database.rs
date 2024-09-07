use std::error::Error;

use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::PgPoolOptions,
    prelude::FromRow,
    types::{
        chrono::{DateTime, Utc},
        uuid::Timestamp,
    },
    PgPool,
};

pub async fn create_pool(
    host: &str,
    username: &str,
    password: &str,
    entry: &str,
) -> Result<PgPool, Box<dyn Error>> {
    let url = format!("postgres://{username}:{password}@{host}/{entry}");
    Ok(PgPoolOptions::new()
        .max_connections(20)
        .connect(&url)
        .await?)
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Users {
    pub id: String,
    pub email: String,
    pub password: String,
    pub dob: String,
    pub username: String,
    pub fullname: String,
    pub address: String,
    pub avatar_url: String,
    pub alias: String,
    pub org_name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
}

#[derive(FromRow)]
pub struct GroupsUsers {
    pub id: String,
    pub user_id: String,
    pub join_date: Timestamp,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}

#[derive(FromRow)]
pub struct Groups {
    pub id: String,
    pub name: String,
    pub creator_id: String,
    pub project_id: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}

#[derive(FromRow)]
pub struct Messages {
    pub id: String,
    pub sender_id: String,
    pub receiver_id: String,
    pub forwarded_from: String,
    pub content: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}

#[derive(FromRow)]
pub struct Projects {
    pub id: String,
    pub mirror_link: String,
    pub owner_id: String,
    pub vir_fs_key: Option<String>,
    pub status: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub deleted_at: Option<Timestamp>,
}
