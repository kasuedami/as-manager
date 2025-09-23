#[cfg(feature = "ssr")]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{postgres::PgQueryResult, prelude::FromRow, PgPool};

#[cfg(feature = "ssr")]
use crate::domain;

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, FromRow)]
pub struct Announcement {
    id: i64,
    title: String,
    content: String,
    hidden: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, FromRow)]
pub struct Team {
    id: i64,
    pub name: String,
    pub contact_person_id: Option<i64>,
    pub platoon_id: Option<i64>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Serialize, Clone, FromRow)]
pub struct Player {
    pub id: i64,
    pub email: String,
    pub tag_name: String,
    pub active: bool,
    pub team_id: Option<i64>,
    pub password_hash: Vec<u8>,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}

#[cfg(feature = "ssr")]
pub async fn save_player(player: domain::Player, pool: &PgPool) -> Result<PgQueryResult, DatabaseError> {
    let query_result = if let Some(id) = player.id {
        sqlx::query("UPDATE Player SET tag_name = $1, email = $2 WHERE id = $3")
            .bind(player.tag_name)
            .bind(player.email)
            .bind(id)
            .execute(pool)
            .await
    } else {
        sqlx::query("INSERT INTO Player (tag_name, email, active, team_id, password_hash) VALUES ($1, $2, $3, $4, '')")
            .bind(player.tag_name)
            .bind(player.email)
            .bind(player.active)
            .bind(player.team_id)
            .execute(pool)
            .await
    };

    match query_result {
        Ok(query_result) => Ok(query_result),
        Err(_) => Err(DatabaseError::Sqlx)
    }
}

#[cfg(feature = "ssr")]
pub async fn create_player(email: String, tag_name: String, password_hash: &[u8], pool: &PgPool) -> Result<(), DatabaseError> {
    let email_user = sqlx::query_as::<_, Player>("SELECT * FROM Player WHERE email LIKE $1")
        .bind(&email)
        .fetch_optional(pool)
        .await;

    match email_user {
        Ok(Some(player)) => return Err(DatabaseError::CreateUserEmailExists(player.email)),
        Err(_) => return Err(DatabaseError::Sqlx),
        _ => ()
    }

    let tag_name_user = sqlx::query_as::<_, Player>("SELECT * FROM Player WHERE tag_name LIKE $1")
        .bind(&tag_name)
        .fetch_optional(pool)
        .await;

    match tag_name_user {
        Ok(Some(player)) => return Err(DatabaseError::CreateUserTagNameExits(player.tag_name)),
        Err(_) => return Err(DatabaseError::Sqlx),
        _ => ()
    }

    let new_player = sqlx::query("INSERT INTO Player (tag_name, email, password_hash, active) VALUES ($1, $2, $3, 'true')")
        .bind(tag_name)
        .bind(email)
        .bind(password_hash)
        .execute(pool)
        .await;

    match new_player {
        Ok(_) => Ok(()),
        Err(_) => Err(DatabaseError::Sqlx)
    }
}

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum DatabaseError {
    #[error("user with email {0} already exists")]
    CreateUserEmailExists(String),
    #[error("user with tag name {0} already exists")]
    CreateUserTagNameExits(String),
    #[error("sqlx error")]
    Sqlx
}
