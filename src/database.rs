#[cfg(feature = "ssr")]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::{prelude::FromRow};
#[cfg(feature = "ssr")]
use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};

#[cfg(feature = "ssr")]
use crate::domain;

#[cfg(feature = "ssr")]
pub mod models;
#[cfg(feature = "ssr")]
pub mod schema;

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
pub fn save_player(player: domain::Player, pool: &DieselPool) -> Result<(), DatabaseError> {
    use diesel::{ExpressionMethods, RunQueryDsl};
    use schema::players::dsl::*;

    diesel::update(players)
        .set((
            tag_name.eq(&player.tag_name),
            email.eq(&player.email),
            active.eq(&player.active),
            team_id.eq(&player.team_id)
        ))
        .execute(&mut pool.get().expect("diesel"))
        .map(|_| ())
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn find_player_for_id(search_id: i64, pool: &DieselPool) -> Result<Option<models::Player>, DatabaseError> {
    use schema::players::dsl::*;
    use diesel::{QueryDsl, RunQueryDsl, OptionalExtension};

    players.find(search_id)
        .get_result(&mut pool.get().expect("diesel"))
        .optional()
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn find_player_for_email(search_email: &str, pool: &DieselPool) -> Result<Option<models::Player>, DatabaseError> {
    use schema::players::dsl::*;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, OptionalExtension};

    players.filter(email.eq(search_email))
        .first::<models::Player>(&mut pool.get().expect("diesel"))
        .optional()
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn create_player(new_player_email: String, new_player_tag_name: String, new_player_password_hash: &[u8], diesel_pool: &DieselPool) -> Result<(), DatabaseError> {
    use models::NewPlayer;
    use schema::players;
    use schema::players::dsl::*;
    use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

    let email_exists_query = diesel::select(
        diesel::dsl::exists(
            players.filter(email.eq(&new_player_email))
        )
    );
    let email_user_exists = email_exists_query.get_result::<bool>(&mut diesel_pool.get().expect("diesel"));

    match email_user_exists {
        Ok(true) => return Err(DatabaseError::CreateUserEmailExists(new_player_email)),
        Err(_) => return Err(DatabaseError::Sqlx),
        _ => ()
    }

    let tag_name_exists_query = diesel::select(
        diesel::dsl::exists(
            players.filter(tag_name.eq(&new_player_tag_name))
        )
    );
    let tag_name_user_exists = tag_name_exists_query.get_result::<bool>(&mut diesel_pool.get().expect("diesel"));

    match tag_name_user_exists {
        Ok(true) => return Err(DatabaseError::CreateUserTagNameExists(new_player_tag_name)),
        Err(_) => return Err(DatabaseError::Sqlx),
        _ => ()
    }

    let new_player = NewPlayer {
        email: &new_player_email,
        tag_name: &new_player_tag_name,
        active: true,
        password_hash: new_player_password_hash
    };

    let new_player = diesel::insert_into(players::table)
        .values(&new_player)
        .returning(models::Player::as_returning())
        .get_result(&mut diesel_pool.get().expect("failed to get diesel connection"));

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
    CreateUserTagNameExists(String),
    #[error("sqlx error")]
    Sqlx,
    #[error("diesel error")]
    Diesel
}

#[cfg(feature = "ssr")]
impl From<diesel::result::Error> for DatabaseError {
    fn from(_: diesel::result::Error) -> Self {
        DatabaseError::Diesel
    }
}

#[cfg(feature = "ssr")]
pub type DieselPool = Pool<ConnectionManager<PgConnection>>;
