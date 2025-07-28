use axum::{extract::{Path, State}, http::{HeaderMap, StatusCode}, response::{Html, IntoResponse, Response}, routing::get, Form, Router};
use serde::Deserialize;

use crate::{app::AppState, domain::{authentication::AuthSession, Announcement}};

use super::make_context;

#[derive(Deserialize)]
struct NewAnnouncement {
    title: String,
    content: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(show_announcements))
        .route("/new", get(show_new).post(do_new))
        .route("/modify/{id}", get(show_modify).post(do_modify))
}

async fn show_announcements(State(state): State<AppState>, auth_session: AuthSession) -> Html<String> {
    let announcements: Vec<Announcement> = sqlx::query_as::<_, Announcement>( "select * from announcement where not hidden order by created_at desc")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    let mut context = make_context("Zug-Orga Ankuendigungen", &auth_session);
    context.insert("announcements", &announcements);

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/announcements.html.tera", &context);

    renderer.unwrap().into()
}

async fn show_new(State(state): State<AppState>, auth_session: AuthSession) -> Html<String> {
    let context = make_context("Zug-Orga Ankuendigung erstellen", &auth_session);

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/announcements/new_modify.html.tera", &context);

    renderer.unwrap().into()
}

async fn show_modify(State(state): State<AppState>, Path(id): Path<i64>, auth_session: AuthSession) -> Html<String> {
    let announcement = sqlx::query_as::<_, Announcement>("select * from announcement where id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let mut context = make_context("Zug-Orga Ankuendigung bearbeiten", &auth_session);
    context.insert("announcement", &announcement);

    let renderer = state.tera
        .read()
        .unwrap()
        .render("pages/announcements/new_modify.html.tera", &context);

    renderer.unwrap().into()
}

async fn do_new(State(state): State<AppState>, Form(new_announcement): Form<NewAnnouncement>) -> Response {
    let insert = sqlx::query("insert into announcement (title, content, hidden) values ($1, $2, false)")
        .bind(new_announcement.title)
        .bind(new_announcement.content)
        .execute(&state.pool).await;

    match insert {
        Ok(_) => {
            let mut header = HeaderMap::new();
            header.insert("HX-Redirect", "/announcements".parse().unwrap());
            (StatusCode::OK, header, "").into_response()
        },
        Err(error) => {
            //TODO: integrate proper logging
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}

async fn do_modify(State(state): State<AppState>, Path(id): Path<i64>, Form(modified_announcement): Form<NewAnnouncement>) -> Response {
    let insert = sqlx::query("update announcement set title = $1, content = $2 where id = $3")
        .bind(modified_announcement.title)
        .bind(modified_announcement.content)
        .bind(id)
        .execute(&state.pool).await;

    match insert {
        Ok(_) => {
            let mut header = HeaderMap::new();
            header.insert("HX-Redirect", "/announcements".parse().unwrap());
            (StatusCode::OK, header, "").into_response()
        },
        Err(error) => {
            //TODO: integrate proper logging
            dbg!(error);
            (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
        }
    }
}
