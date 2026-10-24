mod common;

use web_scraping_example::broken::check;

#[tokio::test]
async fn sorts_links_into_ok_and_broken() -> Result<(), Box<dyn std::error::Error>> {
    let addr = common::serve(&[
        (
            "/index.html",
            "200 OK",
            r#"<a href="/good">good</a><a href="/missing">missing</a><a href="/good">dup</a>"#,
        ),
        ("/good", "200 OK", "fine"),
    ])
    .await;

    let result = check(&format!("http://{addr}/index.html")).await?;

    assert_eq!(result.ok, vec![format!("http://{addr}/good")]);
    assert_eq!(result.broken, vec![format!("http://{addr}/missing")]);
    Ok(())
}

#[tokio::test]
async fn honors_the_base_tag() -> Result<(), Box<dyn std::error::Error>> {
    // The base URL must be absolute, so the page needs to know its own address.
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    drop(listener);
    let page = format!(r#"<base href="http://{addr}/docs/"><a href="page.html">p</a>"#);
    let served = common::serve_on(
        addr,
        &[
            ("/index.html", "200 OK", &page),
            ("/docs/page.html", "200 OK", "fine"),
        ],
    )
    .await;
    assert_eq!(served, addr);

    let result = check(&format!("http://{addr}/index.html")).await?;

    assert_eq!(result.ok, vec![format!("http://{addr}/docs/page.html")]);
    assert!(result.broken.is_empty());
    Ok(())
}

#[tokio::test]
async fn invalid_site_url_is_an_error() {
    assert!(check("not a url").await.is_err());
}
