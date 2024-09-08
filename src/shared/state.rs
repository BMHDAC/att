use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub session: HashMap<String, SessionToken>,
}
