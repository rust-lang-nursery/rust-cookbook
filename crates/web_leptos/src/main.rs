use axum::Router;
use leptos::config::LeptosOptions;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use leptos_router::components::{Route, Router as LeptosRouter, Routes, A};
use leptos_router::hooks::use_params;
use leptos_router::params::Params;
use leptos_router::path;

#[component]
fn App() -> impl IntoView {
    view! {
        <LeptosRouter>
            <nav><A href="/">"Home"</A>" | "<A href="/filter/serde">"serde"</A>" | "<A href="/filter/tokio">"tokio"</A></nav>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("/filter/:query") view=FilterPage />
                </Routes>
            </main>
        </LeptosRouter>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <h1>"Crate Filter"</h1>
        <p>"Visit "<code>"/filter/{query}"</code>" to search the bundled list."</p>
    }
}

#[derive(Params, PartialEq, Clone, Debug)]
struct FilterParams {
    query: Option<String>,
}

#[component]
fn FilterPage() -> impl IntoView {
    let params = use_params::<FilterParams>();
    let query = move || {
        params
            .read()
            .as_ref()
            .ok()
            .and_then(|p| p.query.clone())
            .unwrap_or_default()
    };

    let results = Resource::new(query, filter_crates);

    view! {
        <h1>"Results"</h1>
        <Suspense fallback=|| view! { <p>"Loading..."</p> }>
            {move || results.get().map(|r| match r {
                Ok(items) => view! { <CrateList items /> }.into_any(),
                Err(err) => view! { <p>{format!("error: {err}")}</p> }.into_any(),
            })}
        </Suspense>
    }
}

#[component]
fn CrateList(items: Vec<String>) -> impl IntoView {
    view! {
        <p>{format!("{} match(es)", items.len())}</p>
        <ul>
            {items.into_iter().map(|name| view! { <li>{name}</li> }).collect::<Vec<_>>()}
        </ul>
    }
}

async fn filter_crates(query: String) -> Result<Vec<String>, String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/crates.txt");
    let text = tokio::fs::read_to_string(path)
        .await
        .map_err(|e| e.to_string())?;
    let q = query.to_lowercase();
    Ok(text
        .lines()
        .filter(|line| line.to_lowercase().contains(&q))
        .map(String::from)
        .collect())
}

fn shell(_options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <title>"Crate Filter"</title>
            </head>
            <body><App /></body>
        </html>
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: std::net::SocketAddr = "127.0.0.1:3000".parse()?;
    let conf = LeptosOptions::builder()
        .output_name("web_leptos")
        .site_addr(addr)
        .build();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&conf, routes, {
            let opts = conf.clone();
            move || shell(opts.clone())
        })
        .with_state(conf);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

#[tokio::test]
async fn filters_by_substring() -> Result<(), String> {
    let hits = filter_crates("serde".to_string()).await?;
    assert!(hits.iter().any(|c| c == "serde"));
    assert!(hits.iter().any(|c| c == "serde_json"));
    assert!(!hits.iter().any(|c| c == "tokio"));
    Ok(())
}
