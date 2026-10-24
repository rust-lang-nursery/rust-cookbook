mod common;

use web_scraping_example::links::get_links;

#[tokio::test]
async fn fetches_a_page_and_returns_its_hrefs() -> Result<(), Box<dyn std::error::Error>> {
    let addr = common::serve(&[(
        "/",
        "200 OK",
        r#"<html><body><a href="/one">1</a><a name="skip">2</a><a href="https://example.com/">3</a></body></html>"#,
    )])
    .await;

    let links = get_links(&format!("http://{addr}/")).await?;

    assert_eq!(links, vec!["/one".into(), "https://example.com/".into()]);
    Ok(())
}

#[tokio::test]
async fn unreachable_server_is_an_error() {
    // Port 1 on localhost has no listener.
    assert!(get_links("http://127.0.0.1:1/").await.is_err());
}
