use std::fmt;

use leptos::{prelude::*, server_fn::codec::JsonEncoding};
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{ParentRoute, Route, Router, Routes},
    path,
};
use serde::{Deserialize, Serialize};

use crate::components::{auth::*, play_event::*, player::*, team::*};
use crate::{auth::AuthError, database::DatabaseError};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="h-full">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <Stylesheet id="leptos" href="/pkg/as-manager.css"/>
                <Stylesheet id="leptos" href="/pkg/leptos_tailwind.css"/>
                <MetaTags/>
            </head>
            <body class="h-full">
                <App/>
            </body>
        </html>
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum AppError {
    LeptosError(ServerFnErrorErr),
    Database(DatabaseError),
    AuthError(AuthError),
    MissingContext,
}

impl FromServerFnError for AppError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(value: ServerFnErrorErr) -> Self {
        AppError::LeptosError(value)
    }
}

impl From<ServerFnErrorErr> for AppError {
    fn from(value: ServerFnErrorErr) -> Self {
        AppError::from_server_fn_error(value)
    }
}

impl From<DatabaseError> for AppError {
    fn from(value: DatabaseError) -> Self {
        AppError::Database(value)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="AS-Manager"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=path!("/login") view=Login/>
                    <Route path=path!("/register") view=Register/>
                    <ParentRoute path=path!("/") view=Base>
                        <Route path=path!("") view=LandingPage/>
                        <ParentRoute path=path!("/players") view=Players>
                            <Route path=path!("") view=PlayersTable/>
                            <Route path=path!("new") view=PlayerNew/>
                            <Route path=path!(":id/edit") view=PlayerEdit/>
                            <Route path=path!(":id") view=PlayerProfile/>
                        </ParentRoute>
                        <ParentRoute path=path!("/teams") view=Teams>
                            <Route path=path!("") view=TeamsTable/>
                            <Route path=path!("new") view=TeamNew/>
                            <Route path=path!(":id/edit") view=TeamEdit/>
                            <Route path=path!(":id") view=TeamProfile/>
                        </ParentRoute>
                        <ParentRoute path=path!("/events") view=PlayEvents>
                            <Route path=path!("") view=PlayEventsTable/>
                            <Route path=path!(":id") view=PlayEventDetails/>
                        </ParentRoute>
                    </ParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Base() -> impl IntoView {
    use leptos_router::components::{Outlet, A};

    view! {
        <header class="bg-blue-600 text-white shadow-md sticky top-0 z-50">
            <div class="max-w-7xl mx-auto px-4 py-4 flex justify-between items-center">
                <h1 class="text-xl font-bold">
                    <A href="/" attr:class="hover:underline">"AS-Manager"</A>
                </h1>

                <nav class="space-x-6 text-sm font-medium text-m">
                    <A href="/" attr:class="hover:underline">Home</A>
                    <A href="/events" attr:class="hover:underline">Events</A>
                    <A href="/teams" attr:class="hover:underline">Teams</A>
                    <A href="/players" attr:class="hover:underline">Spieler</A>
                </nav>
            </div>
        </header>

        <main>
            <Outlet/>
        </main>
    }
}

#[component]
fn LandingPage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
        <p>"Double count: " {move || count.get() * 2}</p>
    }
}
