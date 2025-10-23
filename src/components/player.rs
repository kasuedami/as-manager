use leptos::prelude::*;
use leptos::Params;
use leptos_router::{components::A, hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

use crate::domain::Team;
use crate::{app::AppError, domain::Player};
use crate::components::util::{BackButton, BoolSymbol, OptionalLink, SelectFromServer};
use crate::server::{find_team_for_id, get_all_players, get_filtered_teams, find_player_for_id};

#[component]
pub fn Players() -> impl IntoView {
    use crate::components::protected::Protected;
    use leptos_router::components::Outlet;

    view! {
        <Protected>
            <Outlet/>
        </Protected>
    }
}

#[component]
pub fn PlayersTable() -> impl IntoView {

    let players = Resource::new(|| {}, |_| get_all_players());

    view! {
        <BackButton/>
        <div class="p-8 max-w-4xl mx-auto">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">
                    "Spieler"
                </h1>
                <A href="/players/new"
                    attr:class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition"
                >
                    "Neuer Spieler"
                </A>
            </div>

            <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                {
                    move || {
                        players.get().map(|result| match result {
                            Ok(players) => view! {
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
                                                players.into_iter().map(|player: Player| view! {
                                                    <tr class="hover:bg-gray-50">
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/players/{}", player.id.unwrap()) attr:class="hover:underline">{player.id}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/players/{}", player.id.unwrap()) attr:class="hover:underline">{player.tag_name}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/players/{}", player.id.unwrap()) attr:class="hover:underline">{player.email}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <BoolSymbol value=player.active/>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <OptionalLink value=player.team_id
                                                                text=|id| format!("Team Id: {}", id)
                                                                href=|id| format!("/teams/{}", id)
                                                                fallback=move || view! { "Kein Team" }
                                                            />
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
pub fn PlayerProfile() -> impl IntoView {

    let player_id = use_params::<PlayerIdParameter>();
    let player = Resource::new(
        move || player_id.read().clone(),
        move |params_result| find_player_for_id(params_result.unwrap().id),
    );

    let team = Resource::new(
        move || {
            player.get().and_then(|res| res.as_ref().ok().map(|player| player.as_ref().map_or(None, |player| player.team_id))).flatten()
        },
        |id| find_team_for_id(id),
    );

    view! {
        <BackButton/>
        <div class="p-8 max-w-4xl mx-auto">

            <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                {
                    move || {
                        player.get().map(|result| match result {
                            Ok(Some(player)) => view! {
                                <div class="flex items-center justify-between mb-6">
                                    <h1 class="text-2xl font-semibold">
                                        "Spieler " { player.tag_name.clone() }
                                    </h1>
                                    <A href=format!("/players/{}/edit", player.id.unwrap())
                                        attr:class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition"
                                    >
                                        "Bearbeiten"
                                    </A>
                                </div>

                                <div class="space-y-4">

                                    <div class="grid grid-cols-[auto_1fr] items-center gap-4">
                                        <label for="view_player[email]" class="text-left text-gray-700">
                                            "Email:"
                                        </label>
                                        <output
                                            name="view_player[email]"
                                            class="text-left w-full px-3 py-2 focus:outline-none focus:ring focus:border-blue-300">
                                            { player.email }
                                        </output>

                                        <label for="view_player[tag_name]" class="text-left text-gray-700">
                                            "Spielername:"
                                        </label>
                                        <output
                                            name="view_player[tag_name]"
                                            class="text-left w-full px-3 py-2 focus:outline-none focus:ring focus:border-blue-300">
                                            { player.tag_name }
                                        </output>

                                        <label for="view_player[active]" class="text-left text-gray-700">
                                            "Aktiv:"
                                        </label>
                                        <output
                                            name="view_player[active]"
                                            class="text-left w-full px-3 py-2 focus:outline-none focus:ring focus:border-blue-300">
                                            <BoolSymbol value=player.active/>
                                        </output>

                                        <label for="view_player[team]" class="text-left text-gray-700">
                                            "Team:"
                                        </label>
                                        <output
                                            name="view_player[team]"
                                            class="text-left w-full px-3 py-2 focus:outline-none focus:ring focus:border-blue-300">

                                            { move || {
                                                team.get().map(|result| match result {
                                                    Ok(team) => view! {
                                                        <OptionalLink value=team
                                                            text=|team| format!("{}", team.name)
                                                            href=|team| format!("/teams/{}", team.id.unwrap())
                                                            fallback=move || view! { "Kein Team" }
                                                        />
                                                    }.into_any(),
                                                    Err(e) => view! { <p>{ e.to_string() }</p> }.into_any(),
                                                })
                                            }}
                                        </output>
                                    </div>
                                </div>
                            }.into_any(),
                            Ok(None) => view! {
                                "Spieler nicht gefunden"
                            }.into_any(),
                            Err(e) => view! {
                                <p>{ e.to_string() }</p>
                            }.into_any(),
                        })
                    }
                }
            </Suspense>
        </div>
    }
}

#[component]
pub fn PlayerNew() -> impl IntoView {

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
                    <A href="/players"
                        attr:class="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 text-gray-700"
                    >
                        "Abbrechen"
                    </A>
                    <button
                        type="submit"
                        class="px-4 py-2 rounded bg-blue-600 hover:bg-blue-700 text-white"
                    >
                        "Erstellen"
                    </button>
                </div>
            </ActionForm>
        </div>
    }
}

#[component]
pub fn PlayerEdit() -> impl IntoView {

    let player_id = use_params::<PlayerIdParameter>();
    let player = Resource::new(
        move || player_id.read().clone(),
        move |params_result| find_player_for_id(params_result.unwrap().id),
    );

    let team = Resource::new(
        move || {
            player.get().and_then(|res| res.as_ref().ok().map(|player| player.as_ref().map_or(None, |player| player.team_id))).flatten()
        },
        |id| find_team_for_id(id),
    );

    let save_team = ServerAction::<SavePlayer>::new();
    let get_filtered_teams = Action::new(|filter: &String| {
        get_filtered_teams(filter.to_string())
    });

    view! {
        <div class="p-8 max-w-4xl mx-auto">

            <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                {
                    move || {
                        player.get().map(|result| match result {
                            Ok(Some(player)) => view! {
                                <ActionForm action=save_team>
                                    <div class="flex items-center justify-between mb-6">
                                        <h1 class="text-2xl font-semibold">
                                            "Spieler " { player.tag_name.clone() } " bearbeiten"
                                        </h1>
                                    </div>

                                    <div class="space-y-4">

                                        <div class="grid grid-cols-[auto_1fr] items-center gap-4">
                                            <label for="player_form[email]" class="text-left text-gray-700">
                                                "Email:"
                                            </label>
                                            <input
                                                name="player_form[email]"
                                                class="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-300"
                                                value=player.email/>

                                            <label for="player_form[tag_name]" class="text-left text-gray-700">
                                                "Spielername:"
                                            </label>
                                            <input
                                                name="player_form[tag_name]"
                                                class="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-300"
                                                value=player.tag_name/>

                                            <label for="player_form[active]" class="text-left text-gray-700">
                                                "Aktiv:"
                                            </label>
                                            <input
                                                type="checkbox"
                                                name="player_form[active]"
                                                class="w-4 h-4 accent-green-600 border-2 border-gray-300 rounded"
                                                value="true"
                                                checked=player.active/>

                                            <label for="player_form[team_id]" class="text-left text-gray-700">
                                                "Team:"
                                            </label>
                                            { move || {
                                                team.get().map(|result| match result {
                                                    Ok(team) => view! {
                                                        <SelectFromServer
                                                            name="player_form[team_id]"
                                                            current_value=team
                                                            options_action=get_filtered_teams
                                                            option_text=move |team: &Team| team.name.clone()
                                                            default_text="Kein Team"
                                                        />
                                                    }.into_any(),
                                                    Err(e) => view! { <p>{ e.to_string() }</p> }.into_any(),
                                                })
                                            }}

                                            <input
                                                type="hidden"
                                                name="player_form[id]"
                                                value=player.id.unwrap()/>
                                        </div>
                                    </div>

                                    <div class="flex justify-end gap-2 mt-6">
                                        <A href=format!("/players/{}", player.id.unwrap())
                                            attr:class="px-4 py-2 rounded bg-gray-200 hover:bg-gray-300 text-gray-700"
                                        >
                                            "Abbrechen"
                                        </A>
                                        <button
                                            type="submit"
                                            class="px-4 py-2 rounded bg-blue-600 hover:bg-blue-700 text-white"
                                        >
                                            "Speichern"
                                        </button>
                                    </div>
                                </ActionForm>
                            }.into_any(),
                            Ok(None) => view! {
                                <p>"Spieler nicht gefunden"</p>
                            }.into_any(),
                            Err(e) => view! {
                                <p>{ e.to_string() }</p>
                            }.into_any(),
                        })
                    }
                }
            </Suspense>
        </div>
    }
}

#[derive(Params, PartialEq, Clone)]
struct PlayerIdParameter {
    id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateNewPlayerForm {
    email: String,
    tag_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct EditPlayerForm {
    id: i64,
    email: String,
    tag_name: String,
    #[serde(default)]
    active: bool,
    team_id: Option<i64>,
}

#[server]
async fn save_player(player_form: EditPlayerForm) -> Result<(), AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let player = Player {
        id: Some(player_form.id),
        email: player_form.email,
        tag_name: player_form.tag_name,
        active: player_form.active,
        team_id: player_form.team_id,
    };

    let result = database::save_player(player, &pool);

    match result {
        Ok(_) => {
            leptos_axum::redirect("/players");
            Ok(())
        },
        Err(err) => Err(AppError::Database(err)),
    }
}

#[server]
async fn create_new_player(create_new_player: CreateNewPlayerForm) -> Result<(), AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let result = database::create_player(
        create_new_player.email,
        create_new_player.tag_name,
        b"",
        &pool,
    );

    match result {
        Ok(()) => {
            leptos_axum::redirect("/players");
            Ok(())
        },
        Err(err) => Err(AppError::Database(err)),
    }
}
