use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{app::AppError, domain::PrimaryKey};

#[component]
pub fn BackButton() -> impl IntoView {
    use leptos::web_sys;
    use leptos_router::hooks::use_navigate;

    let navigate = use_navigate();

    view! {
        <div class="relative">
            <button class="absolute top-0 left-0 px-4 py-2 mt-2 ml-2 rounded bg-gray-200 hover:bg-gray-300 text-gray-700"
                on:click=move |_| {
                let window = web_sys::window().unwrap();
                let mut path = window.location().pathname().unwrap();

                if path.ends_with('/') && path.len() > 1 {
                    path.pop();
                }

                if let Some(index) = path.rfind('/') {
                    let parent_path = if index == 0 {
                        "/"
                    } else {
                        &path[..index]
                    };

                    navigate(parent_path, Default::default());
                }
            }>
                "Zurück"
            </button>
        </div>
    }
}

#[component]
pub fn BoolSymbol(value: bool) -> impl IntoView {
    if value {
        view! {
            <svg class="h-5 w-5 text-green-600" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M5 13l4 4L19 7" />
            </svg>
        }
    } else {
        view! {
            <svg class="h-5 w-5 text-red-500" fill="none" stroke="currentColor" stroke-width="2" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
            </svg>
        }
    }
}

#[component]
pub fn OptionalLink<T, OlText, OlHref>(
    value: Option<T>,
    text: OlText,
    href: OlHref,
    #[prop(into)] fallback: ViewFnOnce,
) -> impl IntoView
where
    OlText: Fn(&T) -> String,
    OlHref: Fn(&T) -> String,
{
    use leptos_router::components::A;

    match value {
        Some(ref value) => {
            let text = text(value);
            let href = href(value);

            view! {
                <A href=href>
                    {text}
                </A>
            }
            .into_any()
        }
        None => fallback.run(),
    }
}

#[component]
pub fn SelectFromServer<D, F>(
    #[prop(into)] name: String,
    #[prop(default = "None".to_string(), into)] default_text: String,
    #[prop(optional)] current_value: Option<Option<D>>,
    #[prop(into)] options_action: Action<String, Result<Vec<D>, AppError>>,
    option_text: F,
) -> impl IntoView
where 
    D: 'static + Send + Sync + Clone + Serialize + for<'a> Deserialize<'a> + PrimaryKey,
    F: Fn(&D) -> String + Send + Sync + 'static,
{
    let filter = RwSignal::new(String::new());
    let open = RwSignal::new(false);
    let value = RwSignal::new(String::new());

    if let Some(current_value) = current_value.flatten() {
        filter.set(option_text(&current_value));
        value.set(current_value.key().unwrap().to_string());
    }
    Effect::new(move |_| {
        options_action.dispatch(filter.get());
    });

    let options = options_action.value();

    let dropdown_ref = NodeRef::new();

    view! {
        <div
            class="relative w-full"
            tabindex="0"
            on:focusout=move |ev| {
                use leptos::wasm_bindgen::{JsCast, JsValue};
                use leptos::web_sys::{HtmlDivElement, Node};

                let Some(container): Option<HtmlDivElement> = dropdown_ref.get() else { return; };
                let Some(related_target) = ev.related_target() else {
                    open.set(false);
                    return;
                };

                let related_js: JsValue = related_target.into();
                
                let related_node = match related_js.dyn_ref::<Node>() {
                    Some(n) => n,
                    None => {
                        open.set(false);
                        return;
                    }
                };

                if !container.contains(Some(related_node)) {
                    open.set(false);
                }
            }
            node_ref=dropdown_ref
        >
            <input
                type="text"
                autocomplete="off"
                class="w-full px-4 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                prop:value=filter
                on:input:target=move |ev| {
                    filter.set(ev.target().value());
                    open.set(true);
                }
                on:focus=move |_| {
                    open.set(true);
                }
            />
            <input
                type="hidden"
                name=name
                prop:value=value
            />

            <div
                class=move || {
                    if open.get() {
                        "absolute z-10 mt-1 w-full bg-white border border-gray-300 rounded max-h-60 overflow-y-auto"
                    } else {
                        "hidden"
                    }
                }
            >
                <div
                    class="px-4 py-2 text-left bg-gray-200 hover:bg-blue-100 cursor-pointer"
                    on:click=move |_| {
                        filter.set(default_text.clone());
                        value.set(String::new());
                        open.set(false);
                    }
                >
                    { default_text.clone() }
                </div>
                <Show
                    when=move || options.read().is_some()
                    fallback=move || view! { <p>"Loading..."</p> }
                >
                    { match options.get() {
                        Some(options) => match options {
                            Ok(options) => view! {
                                { options.iter().map(|option| {
                                    let text = option_text(option);
                                    let key = option.key().unwrap();

                                    view! {
                                        <div
                                            class="px-4 py-2 text-left hover:bg-blue-100 cursor-pointer"
                                            on:click=move |_| {
                                                filter.set(text.clone());
                                                value.set(key.to_string());
                                                open.set(false);
                                            }
                                        >
                                            { text.clone() }
                                        </div>
                                    }
                                }).collect_view() }
                            }.into_any(),
                            Err(_) => todo!(),
                        },
                        None => todo!(),
                    }}
                </Show>
            </div>
        </div>
    }
}
