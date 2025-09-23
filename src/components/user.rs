use leptos::prelude::*;
use leptos::Params;
use leptos_router::params::Params;

use crate::app::AppError;
use crate::domain::Player;

#[component]
pub fn Users() -> impl IntoView {
    use leptos_router::components::Outlet;
    use crate::components::protected::Protected;

    view! {
        <Protected>
            <h1>Users</h1>
            <Outlet/>
        </Protected>
    }
}

#[component]
pub fn UsersTable() -> impl IntoView {
    view! {
        <h1>"List of all users"</h1>
        <a href="/users/new" class="button">
            Create new user
        </a>
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
    view! {
        <h2>"Create new user"</h2>
        <UserEditOrCreate player=None/>
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
