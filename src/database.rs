#[cfg(feature = "ssr")]
use std::collections::HashSet;

#[cfg(feature = "ssr")]
use diesel::{
    dsl::count_star,
    ExpressionMethods,
    QueryDsl,
    RunQueryDsl,
    SelectableHelper,
    OptionalExtension,
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::domain;

#[cfg(feature = "ssr")]
pub mod models;
#[cfg(feature = "ssr")]
pub mod schema;

#[cfg(feature = "ssr")]
pub fn save_player(player: domain::Player, pool: &DieselPool) -> Result<(), DatabaseError> {
    use schema::players::dsl::*;

    diesel::update(players)
        .filter(id.eq(player.id.unwrap()))
        .set((
            tag_name.eq(&player.tag_name),
            email.eq(&player.email),
            active.eq(&player.active),
            team_id.eq(&player.team_id),
        ))
        .execute(&mut pool.get().expect("diesel"))
        .map(|_| ())
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn find_player_for_id(
    search_id: i64,
    pool: &DieselPool,
) -> Result<Option<models::Player>, DatabaseError> {
    use schema::players::dsl::*;

    players
        .find(search_id)
        .get_result(&mut pool.get().expect("diesel"))
        .optional()
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn find_player_for_email(
    search_email: &str,
    pool: &DieselPool,
) -> Result<Option<models::Player>, DatabaseError> {
    use schema::players::dsl::*;

    players
        .filter(email.eq(search_email))
        .first::<models::Player>(&mut pool.get().expect("diesel"))
        .optional()
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn get_players_for_name_filter(
    filter_name: String,
    pool: &DieselPool
) -> Result<Vec<models::Player>, DatabaseError> {
    use schema::players::dsl::*;

    let mut query = players.into_boxed();

    if !filter_name.is_empty() {
        use diesel::PgTextExpressionMethods;

        let pattern = format!("%{}%", filter_name);
        query = query.filter(tag_name.ilike(pattern));
    }

    Ok(query.load::<models::Player>(&mut pool.get().expect("diesel"))?)
}

#[cfg(feature = "ssr")]
pub fn get_all_players(pool: &DieselPool) -> Result<Vec<models::Player>, DatabaseError> {
    use schema::players;

    players::table
        .load::<models::Player>(&mut pool.get().expect("diesel"))
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn get_players_for_team(filter_team_id: i64, pool: &DieselPool) -> Result<Vec<models::Player>, DatabaseError> {
    use diesel::ExpressionMethods;
    use schema::players::dsl::*;

    let result = players
        .filter(team_id.eq(Some(filter_team_id)))
        .load::<models::Player>(&mut pool.get().expect("diesel"))?;

    Ok(result)
}

#[cfg(feature = "ssr")]
pub fn create_player(
    new_player_email: String,
    new_player_tag_name: String,
    new_player_password_hash: &[u8],
    pool: &DieselPool,
) -> Result<(), DatabaseError> {
    use models::NewPlayer;
    use schema::players::{self, dsl::*};

    let email_exists_query = diesel::select(diesel::dsl::exists(
        players.filter(email.eq(&new_player_email)),
    ));
    let email_user_exists =
        email_exists_query.get_result::<bool>(&mut pool.get().expect("diesel"));

    match email_user_exists {
        Ok(true) => return Err(DatabaseError::CreateUserEmailExists(new_player_email)),
        Err(_) => return Err(DatabaseError::Diesel),
        _ => (),
    }

    let tag_name_exists_query = diesel::select(diesel::dsl::exists(
        players.filter(tag_name.eq(&new_player_tag_name)),
    ));
    let tag_name_user_exists =
        tag_name_exists_query.get_result::<bool>(&mut pool.get().expect("diesel"));

    match tag_name_user_exists {
        Ok(true) => return Err(DatabaseError::CreateUserTagNameExists(new_player_tag_name)),
        Err(_) => return Err(DatabaseError::Diesel),
        _ => (),
    }

    let new_player = NewPlayer {
        email: &new_player_email,
        tag_name: &new_player_tag_name,
        active: true,
        password_hash: new_player_password_hash,
    };

    let new_player = diesel::insert_into(players::table)
        .values(&new_player)
        .returning(models::Player::as_returning())
        .get_result(&mut pool.get().expect("diesel"));

    match new_player {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into()),
    }
}

#[cfg(feature = "ssr")]
pub fn get_all_teams(pool: &DieselPool) -> Result<Vec<models::Team>, DatabaseError> {
    use schema::teams;

    teams::table
        .load::<models::Team>(&mut pool.get().expect("diesel"))
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn find_team_for_id(
    search_id: i64,
    pool: &DieselPool
) -> Result<Option<models::Team>, DatabaseError> {
    use schema::teams::dsl::*;

    teams
        .find(search_id)
        .get_result(&mut pool.get().expect("diesel"))
        .optional()
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn create_team(create_name: String, pool: &DieselPool) -> Result<(), DatabaseError> {
    use models::NewTeam;
    use schema::teams;
    
    let new_team = NewTeam {
        name: &create_name,
        contact_person_id: None,
        platoon_id: None,
    };

    let new_team = diesel::insert_into(teams::table)
        .values(&new_team)
        .returning(models::Team::as_returning())
        .get_result(&mut pool.get().expect("diesel"));

    match new_team {
        Ok(_) => Ok(()),
        Err(err) => Err(err.into())
    }
}

#[cfg(feature = "ssr")]
pub fn save_team(
    team: domain::Team,
    new_member_ids: HashSet<i64>,
    removed_member_ids: HashSet<i64>,
    pool: &DieselPool
) -> Result<(), DatabaseError> {
    let connection = &mut pool.get().expect("diesel");

    {
        use schema::teams::dsl::*;

        diesel::update(teams)
            .filter(id.eq(team.id.unwrap()))
            .set((
                name.eq(&team.name),
                contact_person_id.eq(team.contact_person_id),
                platoon_id.eq(team.platoon_id),
            ))
            .execute(connection)
            .map(|_| ())
            .map_err(DatabaseError::from)?;
    }

    {
        use schema::players::dsl::*;

        let mut new_member_ids = new_member_ids;
        let mut removed_member_ids = removed_member_ids;

        if let Some(contact_person_id) = team.contact_person_id {
            let matches: i64 = players
                .filter(id.eq(contact_person_id))
                .filter(team_id.eq(team.id.unwrap()))
                .select(count_star())
                .first(connection)?;

            if matches == 0 && !new_member_ids.contains(&contact_person_id) {
                new_member_ids.insert(contact_person_id);
            }

            if removed_member_ids.contains(&contact_person_id) {
                removed_member_ids.remove(&contact_person_id);
            }
        }

        if !new_member_ids.is_empty() {
            diesel::update(players)
                .filter(id.eq_any(new_member_ids))
                .set(team_id.eq(team.id))
                .execute(connection)?;
        }

        if !removed_member_ids.is_empty() {
            diesel::update(players)
                .filter(id.eq_any(removed_member_ids))
                .set(team_id.eq(None::<i64>))
                .execute(connection)?;
        }
    }

    Ok(())
}

#[cfg(feature = "ssr")]
pub fn find_platoon_for_id(
    search_id: i64,
    pool: &DieselPool,
) -> Result<Option<models::Platoon>, DatabaseError> {
    use schema::platoons::dsl::*;

    platoons
        .find(search_id)
        .get_result(&mut pool.get().expect("diesel"))
        .optional()
        .map_err(DatabaseError::from)
}

#[cfg(feature = "ssr")]
pub fn get_platoons_for_name_filter(
    filter_name: String,
    pool: &DieselPool
) -> Result<Vec<models::Platoon>, DatabaseError> {
    use schema::platoons::dsl::*;

    let mut query = platoons.into_boxed();

    if !filter_name.is_empty() {
        use diesel::PgTextExpressionMethods;

        let pattern = format!("%{}%", filter_name);
        query = query.filter(name.ilike(pattern));
    }

    Ok(query.load::<models::Platoon>(&mut pool.get().expect("diesel"))?)
}

#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize, PartialEq, Eq)]
pub enum DatabaseError {
    #[error("entity not found")]
    EntityNotFound,
    #[error("user with email {0} already exists")]
    CreateUserEmailExists(String),
    #[error("user with tag name {0} already exists")]
    CreateUserTagNameExists(String),
    #[error("diesel error")]
    Diesel,
}

#[cfg(feature = "ssr")]
impl From<diesel::result::Error> for DatabaseError {
    fn from(_: diesel::result::Error) -> Self {
        DatabaseError::Diesel
    }
}

#[cfg(feature = "ssr")]
pub type DieselPool = Pool<ConnectionManager<PgConnection>>;
