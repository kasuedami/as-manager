use chrono::{DateTime, Utc};
use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = super::schema::players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Player {
    pub id: i64,
    pub email: String,
    pub tag_name: String,
    pub active: bool,
    pub team_id: Option<i64>,
    pub password_hash: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::players)]
pub struct NewPlayer<'a> {
    pub email: &'a str,
    pub tag_name: &'a str,
    pub active: bool,
    pub password_hash: &'a [u8],
}
