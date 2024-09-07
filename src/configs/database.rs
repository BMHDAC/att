use std::error::Error;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, types::uuid::Timestamp, PgPool};

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
    pub dob: NaiveDate,
    pub username: String,
    pub fullname: String,
    pub address: Option<String>,
    pub avatar_url: Option<String>,
    pub alias: Option<String>,
    pub org_name: Option<String>,
    pub status: UserStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize, Deserialize, sqlx::Type, PartialEq, Eq, PartialOrd, Ord)]
#[sqlx(type_name = "user_status", rename_all = "snake_case")]
pub enum UserStatus {
    Clean,
    TempBanned,
    PermaBanned,
}

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "project_status", rename_all = "snake_case")]
pub enum ProjectStatus {
    Clean,
    Dropped,
    Watched,
}

#[derive(sqlx::Type, Serialize, Deserialize)]
#[sqlx(type_name = "group_user_status", rename_all = "snake_case")]
pub enum GroupUserStatus {
    Mod,
    User,
    BlackList,
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
