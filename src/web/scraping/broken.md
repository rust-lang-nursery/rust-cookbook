## Check a webpage for broken links

[![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] [![cat-net-badge]][cat-net]

Call `get_base_url` to retrieve the base URL. If the document has a base tag,
get the href [`attr`] from base tag. [`Position::BeforePath`] of the original
URL acts as a default.

Iterates through links in the document and creates a [`tokio::task::spawn`] task that will 
parse an individual link with [`url::ParseOptions`] and [`Url::parse`]). 
The task makes a request to the links with [reqwest] and verifies
[`StatusCode`].  Then the tasks `await` completion before ending the program.

```rust,edition2024
mod broken {
  {{#include ../../../crates/web/src/broken.rs}}
}

[tokio::main]
fn main() -> anyhow::Result<()> {
    let categorized = broken::check("https://www.rust-lang.org/en-US/").await?;
    println!("OK: {:?}", categorized.ok);
    println!("Broken: {:?}", categorized.broken);
    Ok(())
}
```

[`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[`Position::BeforePath`]: https://docs.rs/url/*/url/enum.Position.html#variant.BeforePath
[`StatusCode`]: https://docs.rs/reqwest/*/reqwest/struct.StatusCode.html
[`tokio::task::spawn`]: https://docs.rs/tokio/*/tokio/task/fn.spawn.html
[`url::Parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`url::ParseOptions`]: https://docs.rs/url/*/url/struct.ParseOptions.html
