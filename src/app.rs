use std::{
    env,
    sync::{Arc, RwLock},
    time::Duration,
};

use axum::{Router, routing::get};
use axum_login::{
    AuthManagerLayerBuilder, login_required,
    tower_sessions::{MemoryStore, SessionManagerLayer},
};
use sqlx::PgPool;
use tera::Tera;
use tera_hot_reload::{LiveReloadLayer, watch};

use crate::{domain::authentication::Backend, routes};

pub async fn run_app() {
    let tera: Arc<RwLock<Tera>> =
        Arc::new(RwLock::new(Tera::new("templates/**/*.html.tera").unwrap()));
    let connection_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&connection_url).await.unwrap();

    let app_state = AppState {
        tera: tera.clone(),
        pool: pool.clone(),
    };

    let livereload = LiveReloadLayer::new();
    let reloader = livereload.reloader();

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(pool);
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = Router::new()
        .layer(livereload)
        .fallback(routes::fallback)
        .route("/", get(routes::index))
        .nest("/announcements", routes::announcements::routes())
        .nest("/platoon", routes::platoon::routes())
        .nest("/profile", routes::profile::routes())
        .nest("/teams", routes::teams::routes())
        .route_layer(login_required!(Backend, login_url = "/login"))
        .merge(routes::login::routes())
        .layer(auth_layer)
        .with_state(app_state);

    let _debouncer = watch(
        move || {
            let _ = tera.write().unwrap().full_reload();
            reloader.reload();
        },
        Duration::from_millis(10),
        vec!["./templates"],
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
pub struct AppState {
    pub tera: Arc<RwLock<Tera>>,
    pub pool: PgPool,
}
