#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use std::env;

    use as_manager::app::*;
    use as_manager::auth::*;
    use as_manager::database::DieselPool;
    use axum::Router;
    use axum_login::{
        tower_sessions::{MemoryStore, SessionManagerLayer},
        AuthManagerLayerBuilder,
    };
    use dotenv::dotenv;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use sqlx::PgPool;
    use diesel::{PgConnection, r2d2::{ConnectionManager, Pool}};

    dotenv().ok();

    let connection_url = env::var("DATABASE_URL").unwrap();
    let pool = PgPool::connect(&connection_url).await.unwrap();

    let manager = ConnectionManager::<PgConnection>::new(connection_url);
    let diesel_pool: DieselPool = Pool::builder()
        .build(manager)
        .expect("failed creating diesel connection pool");

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store);

    let backend = Backend::new(diesel_pool.clone());
    let auth_layer = AuthManagerLayerBuilder::new(backend, session_layer).build();

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options.clone(),
            generate_route_list(App),
            move || {
                provide_context(pool.clone());
                provide_context(diesel_pool.clone());
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .layer(auth_layer)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
