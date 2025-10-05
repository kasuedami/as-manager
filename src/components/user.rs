use leptos::prelude::*;
use leptos::Params;
use leptos_router::params::Params;
use serde::Deserialize;
use serde::Serialize;

use crate::app::AppError;
use crate::domain::Player;

#[component]
pub fn Users() -> impl IntoView {
    use leptos_router::components::Outlet;
    use crate::components::protected::Protected;

    view! {
        <Protected>
            <Outlet/>
        </Protected>
    }
}

#[component]
pub fn UsersTable() -> impl IntoView {
    use leptos_router::components::A;

    use crate::components::util::BoolSymbol;

    let users = Resource::new(|| {}, |_| get_users());

    view! {
        <div class="p-8 max-w-4xl mx-auto">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">
                    "Spieler"
                </h1>
                <A href="/users/new"
                    attr:class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition"
                >
                    "Neuer Spieler"
                </A>
            </div>

            <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                {
                    move || {
                        users.get().map(|result| match result {
                            Ok(users) => view! {
                                <div class="overflow-x-auto">
                                    <table class="min-w-full border border-gray-200 shadow-sm rounded-md bg-white">
                                        <thead class="bg-gray-100 text-gray-700">
                                            <tr>
                                                <th class="text-left py-2 px-4 border-b">Id</th>
                                                <th class="text-left py-2 px-4 border-b">Spilername</th>
                                                <th class="text-left py-2 px-4 border-b">Email</th>
                                                <th class="text-left py-2 px-4 border-b">Aktiv</th>
                                                <th class="text-left py-2 px-4 border-b">Team Id</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {
                                                users.into_iter().map(|user: Player| view! {
                                                    <tr class="hover:bg-gray-50">
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/users/{}", user.id.unwrap()) attr:class="hover:underline">{user.id}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/users/{}", user.id.unwrap()) attr:class="hover:underline">{user.tag_name}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/users/{}", user.id.unwrap()) attr:class="hover:underline">{user.email}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <BoolSymbol value=user.active/>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            {if let Some(team_id) = user.team_id {
                                                                view! { {team_id} }.into_any()
                                                            } else {
                                                                view! { "Kein Team" }.into_any()
                                                            }}
                                                        </th>
                                                    </tr>
                                                }).collect_view()
                                            }
                                        </tbody>
                                    </table>
                                </div>
                            }.into_any(),
                            Err(e) => view! {
                                <p>{e.to_string()}</p>
                            }.into_any(),
                        })
                    }
                }
            </Suspense>
        </div>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    use leptos_router::hooks::use_params;

    let params = use_params::<UserIdParameter>();
    let player = Resource::new(
        move || params.read().clone(),
        move |params_result| load_player_by_id(params_result.unwrap().id.unwrap())
    );

    view! {
        <h2>"Edit user"</h2>
        <Suspense fallback=|| view! { <p>"Loading player"</p> }>
            {move || {
                player.with(|res| {
                    res.as_ref().map(|res| {
                        match res {
                            Ok(player) => view! {
                                <UserEditOrCreate player=Some(player.to_owned())/>
                            }.into_any(),
                            Err(_) => view! {
                                <p>"Error Loading player"</p>
                            }.into_any()
                        }
                    })
                })
            }}
        </Suspense>
    }
}

#[component]
pub fn UserNew() -> impl IntoView {
    use leptos_router::components::A;

    let create_new_player = ServerAction::<CreateNewPlayer>::new();

    view! {
        <div class="p-8 max-w-4xl mx-auto">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">
                    "Neuen Spieler anlegen"
                </h1>
            </div>

            <ActionForm action=create_new_player>
                <div class="space-y-4">

                    <div class="grid grid-cols-[auto_1fr] items-center gap-4">
                        <label for="create_new_player[email]" class="text-left text-gray-700">
                            "Email:"
                        </label>
                        <input
                            type="text"
                            name="create_new_player[email]"
                            class="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-300"
                            required
                        />

                        <label for="create_new_player[tag_name]" class="text-left text-gray-700">
                            "Spielername:"
                        </label>
                        <input
                            type="text"
                            name="create_new_player[tag_name]"
                            class="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-300"
                            required
                        />
                    </div>
                </div>

                <div class="flex justify-end gap-2 mt-6">
                    <A href="/users"
                        attr:class="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 text-gray-700"
                    >
                        "Abort"
                    </A>
                    <button
                        type="submit"
                        class="px-4 py-2 rounded bg-blue-600 hover:bg-blue-700 text-white"
                    >
                        "Confirm"
                    </button>
                </div>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn UserEditOrCreate(player: Option<Player>) -> impl IntoView {
    let id = player.as_ref().map_or(None, |p| p.id);
    let active = player.as_ref().map_or(true.to_string(), |p| p.active.to_string());
    let team_id = player.as_ref().map_or(None, |p| p.team_id);

    let name = player.as_ref().map_or("".to_string(), |p| p.tag_name.clone());
    let email = player.as_ref().map_or("".to_string(),  |p| p.email.clone());

    let save = ServerAction::<SavePlayer>::new();

    view! {
        <ActionForm action=save>
            <input type="hidden" name="player[id]" value=id />
            <input type="hidden" name="player[active]" value=active />
            <input type="hidden" name="player[team_id]" value=team_id />
            <label>"Name:"</label>
            <input id="tag_name" type="text" name="player[tag_name]" prop:value=name/>
            <label>"Email:"</label>
            <input id="email" type="text" name="player[email]" prop:value=email />
            
            <button type="submit">
                {player.is_some().then_some("Update").unwrap_or("Create")}
            </button>
        </ActionForm>
    }
}

#[derive(Params, PartialEq, Clone)]
struct UserIdParameter {
    id: Option<i64>
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateNewPlayerForm {
    email: String,
    tag_name: String,
}

#[server]
async fn get_users() -> Result<Vec<Player>, ServerFnError> {
    use sqlx::PgPool;
    use crate::database;

    let pool = use_context::<PgPool>()
        .ok_or(ServerFnError::new("Missing Database pool in context"))?;

    let database_users = sqlx::query_as::<_, database::Player>("select * from player")
        .fetch_all(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let domain_users = database_users.into_iter()
        .map(|db_user| db_user.into())
        .collect();

    Ok(domain_users)
}

#[server]
async fn load_player_by_id(id: i64) -> Result<Player, ServerFnError> {
    Err(ServerFnError::ServerError("Could not load".to_string()))
}

#[server]
async fn save_player(player: Player) -> Result<(), AppError> {
    use sqlx::PgPool;
    use crate::database;

    let pool = use_context::<PgPool>().ok_or_else(|| AppError::MissingContext)?;

    let result = database::save_player(player, &pool).await;
    match result {
        Err(err) => Err(AppError::Database(err)),
        _ => Ok(())
    }
}

#[server]
async fn create_new_player(create_new_player: CreateNewPlayerForm) -> Result<(), AppError> {
    use crate::database::DieselPool;
    use crate::database;

    let pool = use_context::<DieselPool>().ok_or_else(|| AppError::MissingContext)?;
    let result = database::create_player(create_new_player.email, create_new_player.tag_name, b"", &pool).await;

    match result {
        Ok(()) => Ok(()),
        Err(err) => Err(AppError::Database(err))
    }
}
