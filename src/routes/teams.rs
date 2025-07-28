use axum::{extract::{Path, State}, response::{Html, IntoResponse}, routing::get, Router};
use axum_login::AuthUser;

use crate::{app::AppState, domain::{authentication::AuthSession, Player, Team}};

use super::make_context;



pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(show_teams))
        .route("/{id}", get(show_team).post(edit_team))
        .route("/new", get(show_new_team).post(create_new_team))
}

async fn show_teams(State(state): State<AppState>, auth_session: AuthSession) -> impl IntoResponse {
    let teams = sqlx::query_as::<_, Team>("select * from team")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    let mut context = make_context("Zug-Orga Teams", &auth_session);
    context.insert("teams", &teams);
    
    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/teams.html.tera", &context);

    Html(renderer.unwrap())
}

async fn show_team(State(state): State<AppState>, auth_session: AuthSession, Path(id): Path<i64>) -> impl IntoResponse {
    let team = sqlx::query_as::<_, Team>("select * from team where id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let members = sqlx::query_as::<_, Player>("select * from player where team_id = $1")
        .bind(id)
        .fetch_all(&state.pool)
        .await
        .unwrap();

    let contact_person = if let Some(contact_person_id) = team.contact_person_id {
        members.iter().find(|member| member.id() == contact_person_id)
    } else {
        None
    };

    let mut context = make_context("Zug-Orga Team ansehen", &auth_session);
    context.insert("team", &team);
    context.insert("members", &members);
    context.insert("contact_person", &contact_person);

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/teams/team.html.tera", &context);

    Html(renderer.unwrap())
}

async fn show_new_team(State(state): State<AppState>, auth_session: AuthSession) -> impl IntoResponse {
    let context = make_context("Zug-Orga Neues Team erstellen", &auth_session);

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/teams/team.html.tera", &context);

    Html(renderer.unwrap())
}

async fn edit_team(State(state): State<AppState>, auth_session: AuthSession) -> impl IntoResponse {

}

async fn create_new_team(State(state): State<AppState>, auth_session: AuthSession) -> impl IntoResponse {

}
