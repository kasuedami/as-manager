use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::get,
};
use axum_extra::extract::Form;
use serde::Deserialize;
use tera::Context;

use crate::{
    app::AppState, database, domain::{authentication::AuthSession, platoon::Platoon, Announcement}
};

use super::make_context;

#[derive(Deserialize)]
pub struct NewPlatoonSurvey {
    question: String,
    answers: Vec<String>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(show_platoon))
        .route("/survey/new", get(show_new_survey).post(create_new_survey))
        .route("/survey/new/add-answer", get(add_answer))
        .route("/survey/{id}", get(show_survey))
}

async fn show_platoon(
    State(state): State<AppState>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let platoon = sqlx::query_as::<_, Platoon>("select * from platoon")
        .fetch_one(&state.pool)
        .await
        .unwrap();

    let announcements = sqlx::query_as::<_, Announcement>("select * from announcement")
        .fetch_all(&state.pool)
        .await
        .unwrap();

    let surveys = database::platoon::survey::all_surveys_for_platoon_id(platoon.id(), &state.pool).await;

    let mut context = make_context("Zug-Orga", &auth_session);
    context.insert("platoon", &platoon);
    context.insert("announcements", &announcements);
    context.insert("surveys", &surveys);

    let renderer = state
        .tera
        .read()
        .unwrap()
        .render("pages/platoon.html.tera", &context);

    Html(renderer.unwrap())
}

async fn show_new_survey(
    State(state): State<AppState>,
    auth_session: AuthSession,
) -> impl IntoResponse {
    let context = make_context("Zug-Orga Zug Umfrage erstellen", &auth_session);
    let renderer = state
        .tera
        .read()
        .unwrap()
        .render("pages/platoon/new_survey.html.tera", &context);

    Html(renderer.unwrap())
}

async fn create_new_survey(
    State(state): State<AppState>,
    Form(new_survey): Form<NewPlatoonSurvey>,
) -> impl IntoResponse {
    let mut transaction = state.pool.begin().await.unwrap();

    let platoon_id = sqlx::query_scalar::<_, i64>("select id from platoon")
        .fetch_one(&mut *transaction)
        .await
        .unwrap();

    let (survey_id,) = sqlx::query_as::<_, (i64,)>("insert into platoon_survey (platoon_id) values ($1) returning id")
        .bind(platoon_id)
        .fetch_one(&mut *transaction)
        .await
        .unwrap();

    let (question_id,) = sqlx::query_as::<_, (i64,)>("insert into platoon_survey_question (text, survey_id) values ($1, $2) returning id")
        .bind(new_survey.question)
        .bind(survey_id)
        .fetch_one(&mut *transaction)
        .await
        .unwrap();

    sqlx::query("insert into platoon_survey_question_option (text, question_id) select * from unnest($1::text[], $2::bigint[])")
        .bind(&new_survey.answers)
        .bind(vec![question_id; new_survey.answers.len()])
        .execute(&mut *transaction)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    let mut header = HeaderMap::new();
    header.insert("HX-Redirect", "/platoon".parse().unwrap());
    (StatusCode::OK, header, "").into_response()
}

async fn add_answer(State(state): State<AppState>) -> impl IntoResponse {
    let renderer = state.tera.read().unwrap().render(
        "snippets/platoon/survey/add_answer.html.tera",
        &Context::new(),
    );

    Html(renderer.unwrap())
}

async fn show_survey(
    State(state): State<AppState>,
    auth_session: AuthSession,
    Path(id): Path<i64>,
) -> impl IntoResponse {
}
