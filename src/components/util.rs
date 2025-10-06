use leptos::prelude::*;

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
