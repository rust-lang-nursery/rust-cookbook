## Query the GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Queries [GitHub stargazers API v3][github-api-stargazers] with a
[`reqwest::blocking::Client`] to get list of all users who have marked a GitHub repository with a star. 
[`reqwest::Response`] is deserialized into `User` objects implementing [`serde::Deserialize`].

The program expects the GitHub personal access token to be specified in the
environment variable `GITHUB_TOKEN` and sends it with
[`RequestBuilder::bearer_auth`]. Request setup includes the [`reqwest::header::USER_AGENT`]
header as required by the [GitHub API][github-api]. The program deserializes
the response body with [`Response::json`] into a vector of `User` objects.

```rust,no_run
{{#include ../../../../crates/web/src/bin/rest_get.rs }}
```

[`reqwest::blocking::Client`]: https://docs.rs/reqwest/*/reqwest/blocking/struct.Client.html
[`reqwest::header::USER_AGENT`]: https://docs.rs/reqwest/*/reqwest/header/constant.USER_AGENT.html
[`RequestBuilder::bearer_auth`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.bearer_auth
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`Response::json`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[`serde::Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
[github-api]: https://docs.github.com/en/rest
[github-api-stargazers]: https://docs.github.com/en/rest/activity/starring#list-stargazers
