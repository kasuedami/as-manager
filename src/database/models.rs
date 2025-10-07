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
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::players)]
pub struct NewPlayer<'a> {
    pub email: &'a str,
    pub tag_name: &'a str,
    pub active: bool,
    pub password_hash: &'a [u8],
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = super::schema::teams)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Team {
    pub id: i64,
    pub name: String,
    pub contact_person_id: Option<i64>,
    pub platoon_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Insertable)]
#[diesel(table_name = super::schema::teams)]
pub struct NewTeam<'a> {
    pub name: &'a str,
    pub contact_person_id: Option<i64>,
    pub platoon_id: Option<i64>,
}

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = super::schema::platoons)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Platoon {
    pub id: i64,
    pub team: String,
    pub name: String,
    pub motto: String,
    pub leader_id: Option<i64>,
    pub deputy_leader_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct Announcement {
    id: i64,
    title: String,
    content: String,
    hidden: bool,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
}
