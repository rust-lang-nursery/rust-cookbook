use url::Url;

fn main() -> Result<(), url::ParseError> {
    let url = Url::parse("https://example.com:8080/a/b?x=1&y=two#frag")?;
    println!(
        "host {:?}, port {:?}, path {}",
        url.host_str(),
        url.port(),
        url.path()
    );
    for (key, value) in url.query_pairs() {
        println!("{key} = {value}");
    }
    println!("{}", url.join("../c")?);
    Ok(())
}
