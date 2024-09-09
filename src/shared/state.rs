use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub session: HashMap<String, SessionToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct SessionToken {
    pub expired_date: Option<DateTime<Local>>,
    pub user_agent: String,
    pub device_id: String,
    pub device_type: DeviceType,
}

#[derive(Debug, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
#[serde(rename_all = "camelCase")]
pub enum DeviceType {
    Desktop,
    Mobile,
    Television,
}
