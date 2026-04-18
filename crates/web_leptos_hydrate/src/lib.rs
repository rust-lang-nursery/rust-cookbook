use leptos::prelude::{
    ElementChild, Get, IntoAny, OnAttribute, PropAttribute, Resource, ServerFnError, Set, Suspense,
    event_target_value, signal,
};
use leptos::{IntoView, component, server, view};

#[cfg(feature = "ssr")]
use leptos::prelude::{AutoReload, HydrationScripts};

#[server]
pub async fn filter_crates(query: String) -> Result<Vec<String>, ServerFnError> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/crates.txt");
    let text = tokio::fs::read_to_string(path).await?;
    let q = query.to_lowercase();
    Ok(text
        .lines()
        .filter(|line| line.to_lowercase().contains(&q))
        .map(String::from)
        .collect())
}

#[component]
pub fn App() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let results = Resource::new(
        move || query.get(),
        |q| async move { filter_crates(q).await },
    );

    view! {
        <h1>"Crate search"</h1>
        <input
            type="text"
            placeholder="Type to filter..."
            prop:value=query
            on:input=move |ev| set_query.set(event_target_value(&ev))
        />
        <Suspense fallback=|| view! { <p>"Searching..."</p> }>
            {move || results.get().map(|r| match r {
                Ok(items) => view! {
                    <p>{format!("{} match(es)", items.len())}</p>
                    <ul>
                        {items.into_iter().map(|name| view! { <li>{name}</li> }).collect::<Vec<_>>()}
                    </ul>
                }.into_any(),
                Err(err) => view! { <p>{format!("error: {err}")}</p> }.into_any(),
            })}
        </Suspense>
    }
}

#[cfg(feature = "ssr")]
pub fn shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <title>"Crate search"</title>
            </head>
            <body><App /></body>
        </html>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
