use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod authentication;
pub mod platoon;

#[derive(Debug, Serialize, FromRow)]
pub struct Announcement {
    id: i64,
    title: String,
    content: String,
    hidden: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Team {
    id: i64,
    pub name: String,
    pub contact_person_id: Option<i64>,
    pub platoon_id: Option<i64>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Player {
    id: i64,
    pub email: String,
    pub tag_name: String,
    pub active: bool,
    pub team_id: Option<i64>,
    password_hash: Vec<u8>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NextUri {
    pub next: Option<String>,
}
