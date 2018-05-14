## Check if an API resource exists

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Query the [GitHub Users Endpoint](https://api.github.com/users) using a HEAD request ([`Client::head`]) and then inspect the
response code to determine success. This is a quick way to query a rest resource without
needing to receive a body. You can also configure the [`reqwest::Client`] with [`ClientBuilder::timeout`]
which ensures that a request will not last longer than what is passed to the timeout function.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;

use std::time::Duration;
use reqwest::ClientBuilder;
#
# error_chain! {
#     foreign_links {
#         Reqwest(reqwest::Error);
#     }
# }

fn run() -> Result<()> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

    // The timeout for the request is set to 5 seconds.
    let timeout = Duration::new(5, 0);

    let client = ClientBuilder::new().timeout(timeout).build()?;
    let response = client.head(&request_url).send()?;

    if response.status().is_success() {
        println!("{} is a user!", user);
    } else {
        println!("{} is not a user!", user);
    }

    Ok(())
}
#
# quick_main!(run);
```

[`Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head
[`ClientBuilder::timeout`]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.timeout
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
