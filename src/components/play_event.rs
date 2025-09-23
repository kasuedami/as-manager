use leptos::prelude::*;

#[component]
pub fn PlayEvents() -> impl IntoView {
    use leptos_router::components::Outlet;
    use crate::components::protected::Protected;

    view! {
        <Protected>
            <h1>Events</h1>
            <Outlet/>
        </Protected>
    }
}

#[component]
pub fn PlayEventsTable() -> impl IntoView {
    view! {}
}

#[component]
pub fn PlayEventDetails() -> impl IntoView {
    view! {}
}
