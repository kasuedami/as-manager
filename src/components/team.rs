use leptos::prelude::*;

#[component]
pub fn Teams() -> impl IntoView {
    use crate::components::protected::Protected;
    use leptos_router::components::Outlet;

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
