## Check if an API resource exists

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Query the GitHub Users Endpoint using a HEAD
request ([`Client::head`]) and then inspect the response code to determine
success. This is a quick way to query a rest resource without needing to receive
a body. [`reqwest::Client`] cofigured with [`ClientBuilder::timeout`] ensures
a request will not last longer than a timeout.

```rust,no_run
extern crate reqwest;

use reqwest::Error;
use std::time::Duration;
use reqwest::ClientBuilder;


fn main() -> Result<(), Error> {
    let user = "ferris-the-crab";
    let request_url = format!("https://api.github.com/users/{}", user);
    println!("{}", request_url);

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
```

[`Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head
[`ClientBuilder::timeout`]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.timeout
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
