use std::collections::HashSet;
use std::sync::Arc;

use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::{components::A, hooks::use_params, params::Params};
use serde::{Deserialize, Serialize};

use crate::domain::{Platoon, Player};
use crate::{app::AppError, domain::Team};
use crate::components::util::{BackButton, OptionalLink, SelectFromServer};

#[component]
pub fn Teams() -> impl IntoView {
    use crate::components::protected::Protected;
    use leptos_router::components::Outlet;

    view! {
        <Protected>
            <Outlet/>
        </Protected>
    }
}

#[component]
pub fn TeamsTable() -> impl IntoView {

    let teams = Resource::new(|| {}, |_| get_teams());

    view! {
        <BackButton/>
        <div class="p-8 max-w-4xl mx-auto">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">
                    "Teams"
                </h1>
                <A href="/teams/new"
                    attr:class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition"
                >
                    "Neues Team"
                </A>
            </div>

            <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                {
                    move || {
                        teams.get().map(|result| match result {
                            Ok(teams) => view! {
                                <div class="overflow-x-auto">
                                    <table class="min-w-full border border-gray-200 shadow-sm rounded-md bg-white">
                                        <thead class="bg-gray-100 text-gray-700">
                                            <tr>
                                                <th class="text-left py-2 px-4 border-b">Id</th>
                                                <th class="text-left py-2 px-4 border-b">Name</th>
                                                <th class="text-left py-2 px-4 border-b">Ansprechpartner</th>
                                                <th class="text-left py-2 px-4 border-b">Zug</th>
                                            </tr>
                                        </thead>
                                        <tbody>
                                            {
                                                teams.into_iter().map(|team: Team| view! {
                                                    <tr class="hover:bg-gray-50">
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/teams/{}", team.id.unwrap()) attr:class="hover:underline">{team.id}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <A href=format!("/teams/{}", team.id.unwrap()) attr:class="hover:underline">{team.name}</A>
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <OptionalLink value=team.contact_person_id
                                                                text=|id| format!("{}", id)
                                                                href=|id| format!("/players/{}", id)
                                                                fallback=move || view! { "Kein Ansprechpartner" }
                                                            />
                                                        </th>
                                                        <th class="text-left py-2 px-4 border-b">
                                                            <OptionalLink value=team.platoon_id
                                                                text=|id| format!("Zug: {}", id)
                                                                href=|id| format!("/platoon/{}", id)
                                                                fallback=move || view! { "Kein Zug" }
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
                                <p>{ e.to_string() }</p>
                            }.into_any()
                        })
                    }
                }
            </Suspense>
        </div>
    }
}

#[component]
pub fn TeamProfile() -> impl IntoView {

    let team_id = use_params::<TeamIdParameter>();
    let team = Resource::new(
        move || team_id.read().clone(),
        move |team_id| load_team_by_id(team_id.unwrap().id.unwrap()),
    );

    let members = Resource::new(
        move || team_id.read().clone(),
        move |team_id| get_players_for_team(team_id.unwrap().id.unwrap()),
    );

    let contact_person = Resource::new(
        move || {
            team.get()
                .and_then(|res| res.as_ref().ok().map(|team| team.contact_person_id)).flatten()
        },
        |id| find_player_for_id(id),
    );

    let platoon = Resource::new(
        move || {
            team.get()
                .and_then(|res| res.as_ref().ok().map(|team| team.platoon_id)).flatten()
        },
        |id| find_platoon_for_id(id),
    );


    view! {
        <BackButton/>
        <div class="p-8 max-w-4xl mx-auto">

            <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                {
                    move || {
                        team.get().map(|result| match result {
                            Ok(team) => view! {
                                <div class="flex items-center justify-between mb-6">
                                    <h1 class="text-2xl font-semibold">
                                        "Team " { team.name.clone() }
                                    </h1>
                                    <A href=format!("/teams/{}/edit", team.id.unwrap())
                                        attr:class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition"
                                    >
                                        "Bearbeiten"
                                    </A>
                                </div>

                                <div class="space-y-4">

                                    <div class="grid grid-cols-[auto_1fr] items-center gap-4">
                                        <label for="view_team[name]" class="text-left text-gray-700">
                                            "Name:"
                                        </label>
                                        <output
                                            name="view_team[name]"
                                            class="text-left w-full px-3 py-2 focus:outline-none focus:ring focus:border-blue-300">
                                            { team.name }
                                        </output>

                                        <label for="view_team[contact_person]" class="text-left text-gray-700">
                                            "Ansprechpartner:"
                                        </label>
                                        <output
                                            name="view_team[contact_person]"
                                            class="text-left w-full px-3 py-2 focus:outline-none focus:ring focus:border-blue-300">

                                            {
                                                contact_person.get().map(|result| match result {
                                                    Ok(contact_person) => view! {
                                                        <OptionalLink value=contact_person
                                                            text=|contact_person| format!("{}", contact_person.tag_name)
                                                            href=|contact_person| format!("/players/{}", contact_person.id.unwrap())
                                                            fallback=move || view! { "Kein Ansprechpartner" }
                                                        />
                                                    }.into_any(),
                                                    Err(e) => view! { <p>{ e.to_string() }</p> }.into_any(),
                                                })
                                            }
                                        </output>

                                        <label for="view_team[platoon]" class="text-left text-gray-700">
                                            "Zug:"
                                        </label>
                                        <output
                                            name="view_team[platoon]"
                                            class="text-left w-full px-3 py-2 focus:outline-none focus:ring focus:border-blue-300">

                                            {
                                                platoon.get().map(|result| match result {
                                                    Ok(platoon) => view! {
                                                        <OptionalLink value=platoon
                                                            text=|platoon| format!("{}", platoon.name)
                                                            href=|platoon| format!("/platoons/{}", platoon.id.unwrap())
                                                            fallback=move || view! { "Kein Zug" }
                                                        />
                                                    }.into_any(),
                                                    Err(e) => view! { <p>{ e.to_string() }</p> }.into_any(),
                                                })
                                            }
                                        </output>

                                        <div class="col-span-2">
                                            <MembersTable members=members/>
                                        </div>
                                    </div>
                                </div>
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
pub fn TeamNew() -> impl IntoView {

    let create_new_team = ServerAction::<CreateNewTeam>::new();

    view! {
        <div class="p-8 max-w-4xl mx-auto">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">
                    "Neues Team anlegen"
                </h1>
            </div>

            <ActionForm action=create_new_team>
                <div class="space-y-4">

                    <div class="grid grid-cols-[auto_1fr] items-center gap-4">
                        <label for="create_new_team[name]" class="text-left text-gray-700">
                            "Name:"
                        </label>
                        <input
                            type="text"
                            name="create_new_team[name]"
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
pub fn TeamEdit() -> impl IntoView {

    let team_id = use_params::<TeamIdParameter>();
    let team = Resource::new(
        move || team_id.read().clone(),
        |team_id| load_team_by_id(team_id.unwrap().id.unwrap()),
    );

    let members = RwSignal::new(vec![]);
    let members_resource = Resource::new(
        move || team_id.read().clone(),
        move |team_id| get_players_for_team(team_id.unwrap().id.unwrap()),
    );

    let new_member_ids = RwSignal::new(String::new());
    let removed_member_ids = RwSignal::new(String::new());

    Effect::new(move || {
        let starting_ids: HashSet<i64> = if let Some(Ok(members)) = members_resource.get() {
            members.iter()
                .flat_map(|member: &Player| member.id)
                .collect()
        } else {
            HashSet::new()
        };

        let current_ids: HashSet<i64> = members.get()
            .iter()
            .flat_map(|member: &Player| member.id)
            .collect();

        let new_ids: Vec<String> = current_ids.difference(&starting_ids)
            .cloned()
            .map(|id| id.to_string())
            .collect();
        let removed_ids: Vec<String> = starting_ids.difference(&current_ids)
            .cloned()
            .map(|id| id.to_string())
            .collect();

        new_member_ids.set(new_ids.join(","));
        removed_member_ids.set(removed_ids.join(","));
    });

    let contact_person = Resource::new(
        move || {
            team.get()
                .and_then(|res| res.as_ref().ok().map(|team| team.contact_person_id)).flatten()
        },
        |id| find_player_for_id(id),
    );

    let platoon = Resource::new(
        move || {
            team.get()
                .and_then(|res| res.as_ref().ok().map(|team| team.platoon_id)).flatten()
        },
        |id| find_platoon_for_id(id),
    );

    let save_team = ServerAction::<SaveTeam>::new();
    let get_filtered_players = Action::new(|filter: &String| {
        get_filtered_players(filter.to_string())
    });
    let get_filtered_platoons = Action::new(|filter: &String| {
        get_filtered_platoons(filter.to_string())
    });

    view! {
        <div class="p-8 max-w-4xl mx-auto">

            <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                {move || {
                    team.get().map(|result| match result {
                        Ok(team) => view! {
                            <ActionForm action=save_team>
                                <div class="flex items-center justify-between mb-6">
                                    <h1 class="text-2xl font-semibold">
                                        "Team " { team.name.clone() } " bearbeiten"
                                    </h1>
                                </div>

                                <div class="space-y-4">

                                    <div class="grid grid-cols-[auto_1fr] items-center gap-4">
                                        <label for="team_form[name]"
                                            class="text-left text-gray-700">
                                            "Email:"
                                        </label>
                                        <input
                                            name="team_form[name]"
                                            class="w-full border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-300"
                                            value=team.name/>

                                        <label for="team_form[contact_person_id]"
                                            class="text-left text-gray-700">
                                            "Ansprechpartner:"
                                        </label>
                                        {move || {
                                            contact_person.get().map(|result| match result {
                                                Ok(contact_person) => view! {
                                                    <SelectFromServer
                                                        name="team_form[contact_person_id]"
                                                        current_value=contact_person
                                                        options_action=get_filtered_players
                                                        option_text=Arc::new(move |player: &Player| player.tag_name.clone())
                                                        default_text="Kein Ansprechpartner"
                                                    />
                                                }.into_any(),
                                                Err(e) => view! { <p>{ e.to_string() }</p> }.into_any()
                                            })
                                        }}

                                        <label for="team_form[platoon_id]"
                                            class="text-left text-gray-700">
                                            "Zug:"
                                        </label>
                                        {move || {
                                            platoon.get().map(|result| match result {
                                                Ok(platoon) => view! {
                                                    <SelectFromServer
                                                        name="team_form[platoon_id]"
                                                        current_value=platoon
                                                        options_action=get_filtered_platoons
                                                        option_text=Arc::new(move |platoon: &Platoon| platoon.name.clone())
                                                        default_text="Kein Zug"
                                                    />
                                                }.into_any(),
                                                Err(e) => view! { <p>{ e.to_string() }</p> }.into_any()
                                            })
                                        }}

                                        <div class="col-span-2">
                                            <MembersEditTable members_resource=members_resource members=members/>
                                        </div>

                                        <input
                                            type="hidden"
                                            name="team_form[new_member_ids]"
                                            value=new_member_ids
                                        />

                                        <input
                                            type="hidden"
                                            name="team_form[removed_member_ids]"
                                            value=removed_member_ids
                                        />

                                        <input
                                            type="hidden"
                                            name="team_form[id]"
                                            value=team.id.unwrap()
                                        />
                                    </div>
                                </div>

                                <div class="flex justify-end gap-2 mt-6">
                                    <A href=format!("/teams/{}", team.id.unwrap())
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
                        Err(e) => view! {
                            <p>{ e.to_string() }</p>
                        }.into_any(),
                    })
                }}
            </Suspense>
        </div>
    }
}

#[component]
fn MembersTable(members: Resource<Result<Vec<Player>, AppError>>) -> impl IntoView {
    use super::util::BoolSymbol;

    view! {
        <h2 class="text-left text-xl font-semibold py-2">Mitglieder</h2>

        <div class="overflow-x-auto">
            <table class="min-w-full border border-gray-200 shadow-sm rounded-md bg-white">
                <thead class="bg-gray-100 text-gray-700">
                    <tr>
                        <th class="text-left py-2 px-4 border-b">Id</th>
                        <th class="text-left py-2 px-4 border-b">Spilername</th>
                        <th class="text-left py-2 px-4 border-b">Email</th>
                        <th class="py-2 px-4 border-b">Aktiv</th>
                    </tr>
                </thead>
                <tbody>
                    <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                    { move || {
                        members.get().map(|result| match result {
                            Ok(members) => members
                                .into_iter()
                                .map(|member| view! {
                                    <tr class="hover:bg-gray-50">
                                        <th class="text-left py-2 px-4 border-b">
                                            <A href=format!("/players/{}", member.id.unwrap()) attr:class="hover:underline">{member.id}</A>
                                        </th>
                                        <th class="text-left py-2 px-4 border-b">
                                            <A href=format!("/players/{}", member.id.unwrap()) attr:class="hover:underline">{member.tag_name}</A>
                                        </th>
                                        <th class="text-left py-2 px-4 border-b">
                                            <A href=format!("/players/{}", member.id.unwrap()) attr:class="hover:underline">{member.email}</A>
                                        </th>
                                        <th class="py-2 px-4 border-b">
                                            <div class="flex justify-center">
                                                <BoolSymbol value=member.active/>
                                            </div>
                                        </th>
                                    </tr>
                                })
                                .collect_view().into_any(),
                            Err(e) => view! {
                                <p>{ e.to_string() }</p>
                            }.into_any(),
                        })
                    }}
                    </Suspense>
                </tbody>
            </table>
        </div>
    }
}

#[component]
fn MembersEditTable(members_resource: Resource<Result<Vec<Player>, AppError>>, members: RwSignal<Vec<Player>>) -> impl IntoView {
    use super::util::BoolSymbol;

    Effect::new(move |_| {
        if let Some(Ok(data)) = members_resource.get() {
            members.set(data);
        }
    });

    let (new_member_email, set_new_member_email) = signal(String::new());

    let add_member = move |_| {
        let email = new_member_email.get();

        spawn_local(async move {
            let new_member = find_player_for_email(email).await;

            match new_member {
                Ok(Some(player)) => {
                    members.update(|m| m.push(player));
                },
                _ => ()
            }
        });
    };

    let remove_member = move |id: i64| {
        members.update(|members|
            members.retain(|member| member.id.unwrap() != id));
    };

    view! {
        <div class="py-2 flex items-center justify-between">
            <h2 class="text-left text-xl font-semibold py-2">Mitglieder</h2>
            <div class="w-1/2 flex">
                <input
                    class="flex-grow px-4 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                    prop:value=new_member_email
                    on:input:target=move |ev| {
                        set_new_member_email.set(ev.target().value());
                    }
                />
                <button
                    type="button"
                    class="px-4 py-2 ml-2 rounded bg-blue-600 hover:bg-blue-700 text-white"
                    on:click=add_member>
                        "Hinzufügen"
                </button>
            </div>
        </div>

        <div class="overflow-x-auto">
            <table class="table-auto min-w-full border border-gray-200 shadow-sm rounded-md bg-white">
                <thead class="bg-gray-100 text-gray-700">
                    <tr>
                        <th class="text-left py-2 px-4 border-b">Id</th>
                        <th class="text-left py-2 px-4 border-b">Spilername</th>
                        <th class="text-left py-2 px-4 border-b">Email</th>
                        <th class="py-2 px-4 border-b">Aktiv</th>
                        <th class="py-2 px-4 border-b max-w-min">Entfernen</th>
                    </tr>
                </thead>
                <tbody>
                    <Suspense fallback=move || view! { <p>"Lade Daten..."</p> }>
                    { move || match members_resource.get() {
                        Some(Ok(_)) => {
                            members
                                .get()
                                .into_iter()
                                .map(|member| view! {
                                    <tr class="hover:bg-gray-50">
                                        <th class="text-left py-2 px-4 border-b">
                                            <A href=format!("/players/{}", member.id.unwrap()) attr:class="hover:underline">{member.id}</A>
                                        </th>
                                        <th class="text-left py-2 px-4 border-b">
                                            <A href=format!("/players/{}", member.id.unwrap()) attr:class="hover:underline">{member.tag_name}</A>
                                        </th>
                                        <th class="text-left py-2 px-4 border-b">
                                            <A href=format!("/players/{}", member.id.unwrap()) attr:class="hover:underline">{member.email}</A>
                                        </th>
                                        <th class="py-2 px-4 border-b">
                                            <div class="flex justify-center">
                                                <BoolSymbol value=member.active/>
                                            </div>
                                        </th>
                                        <th class="py-2 px-4 border-b max-w-min">
                                            <button
                                                type="button"
                                                class="px-4 py-2 rounded bg-blue-600 hover:bg-blue-700 text-white"
                                                on:click=move |_| remove_member(member.id.unwrap())>
                                                    "Entfernen"
                                            </button>
                                        </th>
                                    </tr>
                                })
                                .collect_view().into_any()
                            },
                        Some(Err(e)) => view! {
                            <p>{ e.to_string() }</p>
                        }.into_any(),
                        None => view! { "Lade Daten..." }.into_any(),
                    }}
                    </Suspense>
                </tbody>
            </table>
        </div>
    }
}

#[derive(Params, PartialEq, Clone)]
struct TeamIdParameter {
    id: Option<i64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct CreateNewTeamForm {
    name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct EditTeamForm {
    id: i64,
    name: String,
    #[serde(default)]
    contact_person_id: Option<i64>,
    #[serde(default)]
    platoon_id: Option<i64>,
    #[serde(default)]
    new_member_ids: String,
    #[serde(default)]
    removed_member_ids: String,
}

#[server]
async fn get_teams() -> Result<Vec<Team>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_teams = database::get_all_teams(&pool)?;
    let domain_teams = database_teams
        .into_iter()
        .map(|db_team| db_team.into())
        .collect();

    Ok(domain_teams)
}

#[server]
async fn load_team_by_id(id: i64) -> Result<Team, AppError> {
    use crate::database::{self, DatabaseError, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let result = database::find_team_for_id(id, &pool);

    match result {
        Ok(Some(team)) => Ok(team.into()),
        Ok(None) => Err(DatabaseError::EntityNotFound.into()),
        Err(err) => Err(err.into()),
    }
}

#[server]
async fn create_new_team(create_new_team: CreateNewTeamForm) -> Result<(), AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let result = database::create_team(create_new_team.name, &pool);

    match result {
        Ok(()) => {
            leptos_axum::redirect("/teams");
            Ok(())
        },
        Err(err) => Err(err.into())
    }
}

#[server]
async fn save_team(team_form: EditTeamForm) -> Result<(), AppError> {
    use crate::database::{self, DieselPool};

    let new_member_ids: HashSet<i64> = team_form.new_member_ids
        .split(",")
        .flat_map(|id| id.parse().ok())
        .collect();
    let removed_member_ids: HashSet<i64> = team_form.removed_member_ids
        .split(",")
        .flat_map(|id| id.parse().ok())
        .collect();

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let team = Team {
        id: Some(team_form.id),
        name: team_form.name,
        contact_person_id: team_form.contact_person_id,
        platoon_id: team_form.platoon_id,
    };

    database::save_team(team, new_member_ids, removed_member_ids, &pool)?;

    leptos_axum::redirect("/teams");
    Ok(())
}

#[server]
async fn get_all_players() -> Result<Vec<Player>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_players = database::get_all_players(&pool)?;
    let domain_players = database_players
        .into_iter()
        .map(|db_player| db_player.into())
        .collect();

    Ok(domain_players)
}

#[server]
async fn get_filtered_players(filter: String) -> Result<Vec<Player>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_players = database::get_players_for_name_filter(filter, &pool)?;
    let domain_players = database_players
        .into_iter()
        .map(|db_player| db_player.into())
        .collect();

    Ok(domain_players)
}


#[server]
async fn find_player_for_id(id: Option<i64>) -> Result<Option<Player>, AppError> {
    use crate::database::{self, DieselPool};

    if id.is_none() {
        return Ok(None)
    }

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_player = database::find_player_for_id(id.unwrap(), &pool)?;
    let domain_player = database_player
        .map(|db_player| db_player.into());

    Ok(domain_player)
}

#[server]
async fn get_filtered_platoons(filter: String) -> Result<Vec<Platoon>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_platoons = database::get_platoons_for_name_filter(filter, &pool)?;
    let domain_platoons = database_platoons
        .into_iter()
        .map(|db_platoon| db_platoon.into())
        .collect();

    Ok(domain_platoons)
}

#[server]
async fn find_platoon_for_id(id: Option<i64>) -> Result<Option<Platoon>, AppError> {
    use crate::database::{self, DieselPool};

    if id.is_none() {
        return Ok(None)
    }

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_platoon = database::find_platoon_for_id(id.unwrap(), &pool)?;
    let domain_platoon = database_platoon
        .map(|db_platoon| db_platoon.into());

    Ok(domain_platoon)
}

#[server]
async fn get_players_for_team(team_id: i64) -> Result<Vec<Player>, AppError> {
    use crate::database::{self, DieselPool};

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_players = database::get_players_for_team(team_id, &pool)?;
    let domain_players = database_players
        .into_iter()
        .map(|db_player| db_player.into())
        .collect();

    Ok(domain_players)
}

#[server]
async fn find_player_for_email(email: String) -> Result<Option<Player>, AppError> {
    use crate::database::{self, DieselPool};

    if email.is_empty() {
        return Ok(None)
    }

    let pool = use_context::<DieselPool>()
        .ok_or_else(|| AppError::MissingContext)?;

    let database_player = database::find_player_for_email(&email, &pool);
    let domain_player = database_player
        .map(|db_player| db_player.map(Into::into));

    Ok(domain_player?)
}
