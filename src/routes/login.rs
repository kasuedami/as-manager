use axum::{extract::{Query, State}, http::{HeaderMap, StatusCode}, response::{Html, IntoResponse}, routing::get, Form, Router};

use crate::{app::AppState, domain::{authentication::{AuthSession, Credentials}, NextUri}};

use super::make_context;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/login", get(show_login).post(do_login))
}

async fn show_login(State(state): State<AppState>, auth_session: AuthSession, Query(NextUri { next }): Query<NextUri>) -> impl IntoResponse {
    let mut context = make_context("Zug-Orga Login", &auth_session);
    context.insert("redirect_to", &next.unwrap_or("/announcements".to_string()));

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/login.html.tera", &context);

    Html(renderer.unwrap())
}

async fn do_login(mut auth_session: AuthSession, Form(credentials): Form<Credentials>) -> impl IntoResponse {
    let redirect_to = credentials.redirect_to.parse().unwrap();

    let player = match auth_session.authenticate(credentials).await {
        Ok(Some(player)) => player,
        Ok(None) => return StatusCode::UNAUTHORIZED.into_response(),
        Err(_) => return StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    };

    if auth_session.login(&player).await.is_err() {
        return StatusCode::INTERNAL_SERVER_ERROR.into_response();
    }

    let mut header = HeaderMap::new();
    header.insert("HX-Redirect", redirect_to);
    (StatusCode::OK, header, "").into_response()
}
