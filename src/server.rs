use leptos::prelude::*;

use crate::{app::AppError, domain::{Platoon, Player, Team}};

#[server]
pub async fn get_all_players() -> Result<Vec<Player>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_players = database::get_all_players(&pool)?;
    let domain_players = database_players
        .into_iter()
        .map(|db_player| db_player.into())
        .collect();

    Ok(domain_players)
}

#[server]
pub async fn get_filtered_players(filter: String) -> Result<Vec<Player>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_players = database::get_players_for_name_filter(filter, &pool)?;
    let domain_players = database_players
        .into_iter()
        .map(|db_player| db_player.into())
        .collect();

    Ok(domain_players)
}

#[server]
pub async fn find_player_for_id(id: Option<i64>) -> Result<Option<Player>, AppError> {
    use crate::database::{self, DieselPool};

    if id.is_none() {
        return Ok(None)
    }

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_player = database::find_player_for_id(id.unwrap(), &pool)?;
    let domain_player = database_player
        .map(|db_player| db_player.into());

    Ok(domain_player)
}

#[server]
pub async fn find_player_for_email(email: String) -> Result<Option<Player>, AppError> {
    use crate::database::{self, DieselPool};

    if email.is_empty() {
        return Ok(None)
    }

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_player = database::find_player_for_email(&email, &pool);
    let domain_player = database_player
        .map(|db_player| db_player.map(Into::into));

    Ok(domain_player?)
}

#[server]
pub async fn find_team_for_id(id: Option<i64>) -> Result<Option<Team>, AppError> {
    use crate::database::{self, DieselPool};

    if id.is_none() {
        return Ok(None)
    }

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_team = database::find_team_for_id(id.unwrap(), &pool)?;
    let domain_team = database_team.map(|db_team| db_team.into());

    Ok(domain_team)
}

#[server]
pub async fn get_filtered_teams(filter: String) -> Result<Vec<Team>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_teams = database::get_teams_for_name_filter(filter, &pool)?;
    let domain_teams = database_teams
        .into_iter()
        .map(|db_team| db_team.into())
        .collect();

    Ok(domain_teams)
}

#[server]
pub async fn get_players_for_team(team_id: i64) -> Result<Vec<Player>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_players = database::get_players_for_team(team_id, &pool)?;
    let domain_players = database_players
        .into_iter()
        .map(|db_player| db_player.into())
        .collect();

    Ok(domain_players)
}

#[server]
pub async fn get_filtered_platoons(filter: String) -> Result<Vec<Platoon>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_platoons = database::get_platoons_for_name_filter(filter, &pool)?;
    let domain_platoons = database_platoons
        .into_iter()
        .map(|db_platoon| db_platoon.into())
        .collect();

    Ok(domain_platoons)
}

#[server]
pub async fn find_platoon_for_id(id: Option<i64>) -> Result<Option<Platoon>, AppError> {
    use crate::database::{self, DieselPool};

    if id.is_none() {
        return Ok(None)
    }

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_platoon = database::find_platoon_for_id(id.unwrap(), &pool)?;
    let domain_platoon = database_platoon
        .map(|db_platoon| db_platoon.into());

    Ok(domain_platoon)
}

