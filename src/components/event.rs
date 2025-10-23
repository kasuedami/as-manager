use leptos::{logging::log, prelude::*};
use leptos_router::components::A;
use serde::{Deserialize, Serialize};

use crate::{app::AppError, components::util::BackButton};

#[component]
pub fn Events() -> impl IntoView {
    use crate::components::protected::Protected;
    use leptos_router::components::Outlet;

    view! {
        <Protected>
            <Outlet/>
        </Protected>
    }
}

#[component]
pub fn EventsTable() -> impl IntoView {
    view! {
        <BackButton/>
        <div class="p-8 max-w-4xl mx-auto">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">
                    "Events"
                </h1>
                <A href="/events/new"
                    attr:class="bg-blue-600 hover:bg-blue-700 text-white font-medium py-2 px-4 rounded transition"
                >
                    "Neues Event"
                </A>
            </div>
        </div>
    }
}

#[component]
pub fn EventNew() -> impl IntoView {
    view! {
        <div class="p-8 max-w-4xl mx-auto">
            <div class="flex items-center justify-between mb-6">
                <h1 class="text-2xl font-semibold">
                    "Neues Event anlegen"
                </h1>
            </div>

            <div>
                <label for="new_event[start_date]">
                    "Datum:"
                </label>
                <input type="date" name="new_event[start_date]"/>
                <label for="new_event[start_time]">
                    "Anfang:"
                </label>
                <input type="time" name="new_event[start_time]"/>
            </div>
        </div>
    }
}

#[component]
pub fn EventDetails() -> impl IntoView {
    view! {}
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct NewEventForm {
    name: String,
    description: String,
    start_date: String,
    start_time: String,
    end_date: String,
    end_time: String,
}

#[server]
async fn new_event(new_event: NewEventForm) -> Result<(), AppError> {
    todo!()
}
