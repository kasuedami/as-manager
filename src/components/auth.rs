use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos_axum::extract;
use leptos_router::components::Redirect;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::auth::AuthSession;
#[cfg(feature = "ssr")]
use crate::database::create_player;
use crate::{app::AppError, auth::Credentials};

#[component]
pub fn Login() -> impl IntoView {
    let do_login = ServerAction::<DoLogin>::new();

    view! {
        <div class="flex justify-center items-start h-screen pt-32 bg-gray-100">
            <div class="bg-white shadow-lg rounded p-8 w-full max-w-md">
                <ActionForm action=do_login.clone() attr:class="space-y-6">

                    <div class="grid grid-cols-[auto_1fr] items-center gap-4">
                        <label for="credentials[email]" class="text-left text-gray-700">
                            "Email:"
                        </label>
                        <input id="credentials[email]"
                            name="credentials[email]"
                            type="text"
                            class="flex-1 border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-500"
                        />

                        <label for="credentials[password]" class="text-left text-gray-700">
                            "Password:"
                        </label>
                        <input id="credentials[password]"
                            name="credentials[password]"
                            type="password"
                            class="flex-1 border border-gray-300 rounded px-3 py-2 focus:outline-none focus:ring focus:border-blue-500"
                        />
                    </div>

                    <div class="flex justify-end">
                        <button
                            type="submit"
                            class="bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-6 rounded"
                        >
                            "Login"
                        </button>
                    </div>
                    <div>
                        {
                            move || match do_login.value().get() {
                                Some(Ok(())) => view! { <Redirect path="/"/> }.into_any(),
                                Some(Err(AppError::AuthError(err))) => view! { <p>"Error: " {err.to_string()}</p> }.into_any(),
                                Some(Err(_)) => view! { <p>"Error"</p> }.into_any(),
                                None => view! { <p>"Waiting for submission..."</p> }.into_any()
                            }
                        }
                    </div>
                </ActionForm>
            </div>
        </div>
    }
}

#[component]
pub fn Register() -> impl IntoView {
    let do_register = ServerAction::<DoRegister>::new();

    view! {
        <ActionForm action=do_register>
            <label>Email</label>
            <input name="register_form[email]" type="text" />

            <label>Tag name</label>
            <input name="register_form[tag_name]" type="text" />

            <label>Password</label>
            <input name="register_form[password]" type="password" />

            <button type="submit">"Register"</button>
        </ActionForm>

        <div>
            {
                move || match do_register.value().get() {
                    Some(Ok(true)) => view! { <Redirect path="/"/> }.into_any(),
                    Some(Ok(false)) => view! { <p>"False"</p> }.into_any(),
                    Some(Err(AppError::Database(err))) => view! {
                        <p>{err.to_string()}</p>
                    }.into_any(),
                    Some(Err(_)) => view! { <p>"Some other error"</p> }.into_any(),
                    None => view! { <p>"Waiting for submission..."</p> }.into_any()
                }
            }
        </div>
    }
}

#[server]
async fn do_login(credentials: Credentials) -> Result<(), AppError> {
    use crate::auth::AuthError;

    let mut auth_session: AuthSession = extract().await?;

    let player = match auth_session.authenticate(credentials).await {
        Ok(Some(player)) => player,
        Ok(None) => return Err(AppError::AuthError(AuthError::InvalidLogin)),
        Err(_) => return Err(AppError::AuthError(AuthError::Backend)),
    };

    if auth_session.login(&player).await.is_err() {
        return Err(AppError::AuthError(AuthError::Backend));
    }

    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct RegisterForm {
    email: String,
    tag_name: String,
    password: String,
}

#[server]
async fn do_register(register_form: RegisterForm) -> Result<bool, AppError> {
    use crate::database::DieselPool;
    use crypto_hashes::sha3::{Digest, Sha3_512};

    let pool = use_context::<DieselPool>().ok_or_else(|| AppError::MissingContext)?;
    let mut hasher = Sha3_512::default();
    hasher.update(register_form.password);
    let password_hash = format!("{:x}", hasher.finalize());

    let register = create_player(
        register_form.email,
        register_form.tag_name,
        &password_hash.into_bytes(),
        &pool,
    );

    match register {
        Ok(()) => Ok(true),
        Err(err) => Err(AppError::Database(err)),
    }
}
