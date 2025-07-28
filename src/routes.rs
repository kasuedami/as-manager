use axum::{
    extract::State, response::{Html, IntoResponse, Redirect, Response}
};
use tera::Context;

use crate::{app::AppState, domain::authentication::AuthSession};

pub mod announcements;
pub mod login;
pub mod platoon;
pub mod profile;
pub mod teams;

pub async fn fallback(State(state): State<AppState>, uri: axum::http::Uri, auth_session: AuthSession) -> Response {
    let mut context = make_context("Zug-Orga page not found", &auth_session);
    context.insert("request_path", uri.path());

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/fallback.html.tera", &context);
    (axum::http::StatusCode::NOT_FOUND, Html(renderer.unwrap())).into_response()
}

pub async fn index() -> Response {
    Redirect::to("/login").into_response()
}


pub fn make_context(page_title: &str, auth_session: &AuthSession) -> Context {
    let mut context = Context::new();
    context.insert("title", page_title);
    context.insert("loggedIn", &auth_session.user.is_some());

    context
}
