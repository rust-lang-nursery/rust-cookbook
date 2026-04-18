#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use web_leptos_hydrate::{App, shell};

    let conf = get_configuration(None)?;
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&options, routes, {
            let opts = options.clone();
            move || shell(opts.clone())
        })
        .with_state(options);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {}
