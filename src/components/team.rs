use leptos::prelude::*;

#[component]
pub fn Teams() -> impl IntoView {
    use leptos_router::components::Outlet;
    use crate::components::protected::Protected;

    view! {
        <Protected>
            <h1>Teams</h1>
            <Outlet/>
        </Protected>
    }
}

#[component]
pub fn TeamsTable() -> impl IntoView {
    view! {}
}

#[component]
pub fn TeamProfile() -> impl IntoView {
    view! {}
}
