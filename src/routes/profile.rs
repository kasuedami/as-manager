use axum::{extract::State, http::{HeaderMap, StatusCode}, response::{Html, IntoResponse}, routing::{get, post}, Form, Router};
use axum_login::AuthUser;
use serde::Deserialize;

use crate::{app::AppState, domain::{authentication::AuthSession, Team}};

use super::make_context;

#[derive(Deserialize)]
struct UserUpdate {
    tag_name: String,
    email: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(show_profile))
        .route("/save", post(do_save))
}

async fn show_profile(State(state): State<AppState>, auth_session: AuthSession) -> impl IntoResponse {
    let mut context = make_context("Zug-Orga Profil", &auth_session);
    let player = auth_session.user.unwrap();
    context.insert("player", &player);

    if let Some(team_id) = player.team_id {
        let team = sqlx::query_as::<_, Team>("select * from team where id = $1")
            .bind(team_id)
            .fetch_one(&state.pool)
            .await;

        match team {
            Ok(team) => context.insert("team", &team),
            Err(err) => {
                tracing::error!(error = %err, "Failed to load team");
                return (StatusCode::INTERNAL_SERVER_ERROR).into_response();
            }
        }
    }

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/profile.html.tera", &context);

    Html(renderer.unwrap()).into_response()
}

async fn do_save(State(state): State<AppState>, auth_session: AuthSession, Form(user_update): Form<UserUpdate>) -> impl IntoResponse {
    let result = sqlx::query("update player set email = $1, tag_name = $2 where id = $3")
        .bind(user_update.email)
        .bind(user_update.tag_name)
        .bind(auth_session.user.unwrap().id())
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => {
            let mut header = HeaderMap::new();
            header.insert("HX-Redirect", "/announcements".parse().unwrap());
            (StatusCode::OK, header, "").into_response()
        },
        Err(err) => {
            tracing::error!(error = %err, "Failed to update user");
            (StatusCode::INTERNAL_SERVER_ERROR).into_response()
        },
    }
}
