use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::extract;
use leptos_router::components::Redirect;

#[cfg(feature = "ssr")]
use crate::auth::AuthSession;

#[component]
pub fn Protected(children: ChildrenFn) -> impl IntoView {
    let access_check = Resource::new(|| {}, |_| check_user_logged_in());

    view! {
        <Suspense>
            {
                move || match *access_check.read() {
                    Some(Ok(true)) => {
                        children().into_any()
                    },
                    Some(Ok(false)) | Some(Err(_)) => {
                        view ! {
                            <Redirect path="/login"/>
                        }.into_any()
                    },
                    None => view! {}.into_any()
                }
            }
        </Suspense>
    }
}

#[server]
async fn check_user_logged_in() -> Result<bool, ServerFnError> {
    let auth_session: AuthSession = extract().await?;
    Ok(auth_session.user.is_some())
}
