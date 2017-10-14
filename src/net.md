# Networking

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Parse a URL from a string to a `Url` type][ex-url-parse] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create a base URL by removing path segments][ex-url-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create new URLs from a base URL][ex-url-new-from-base] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract the URL origin (scheme / host / port)][ex-url-origin] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Remove fragment identifiers and query pairs from a URL][ex-url-rm-frag] | [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Make a HTTP GET request][ex-url-basic] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Download a file to a temporary directory][ex-url-download] | [![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] | [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem] |
| [Query the GitHub API][ex-rest-get] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Consume a paginated RESTful API][ex-paginated-api] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [Check if an API resource exists][ex-rest-head] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Set custom headers and URL parameters for a REST request][ex-rest-custom-params] | [![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Create and delete Gist with GitHub API][ex-rest-post] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [POST a file to paste-rs][ex-file-post] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Listen on unused port TCP/IP][ex-random-port-tcp] | [![std-badge]][std] | [![cat-net-badge]][cat-net] |
| [Extract all links from a webpage HTML][ex-extract-links-webpage] | [![reqwest-badge]][reqwest] [![select-badge]][select] | [![cat-net-badge]][cat-net] |
| [Check webpage for broken links][ex-check-broken-links] | [![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] | [![cat-net-badge]][cat-net] |
| [Extract all unique links from a MediaWiki markup][ex-extract-mediawiki-links] | [![reqwest-badge]][reqwest] [![regex-badge]][regex] | [![cat-net-badge]][cat-net] |
| [Make a partial download with HTTP range headers][ex-progress-with-range] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Handling the Github rate limit error condition][ex-rate-limit-exceeded] | [![reqwest-badge]][reqwest] [![hyper-badge]][hyper] | [![cat-net-badge]][cat-net] |

[ex-url-parse]: #ex-url-parse
<a name="ex-url-parse"/>
## Parse a URL from a string to a `Url` type

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`parse`] method from the `url` crate validates and parses a `&str` into a
[`Url`] struct. The input string may be malformed so this method returns
`Result<Url, ParseError>`.

Once the URL has been parsed, it can be used with all of the methods on the
`Url` type.

The URL in this code parses successfully, but swapping it out for a malformed
URL will print a message containing an explanation of what went wrong.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::Url;
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }

fn run() -> Result<()> {
    let s = "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open";

    let parsed = Url::parse(s)?;
    println!("The path part of the URL is: {}", parsed.path());

    Ok(())
}
#
# quick_main!(run);
```

[ex-url-base]: #ex-url-base
<a name="ex-url-base"></a>
## Create a base URL by removing path segments

[![url-badge]][url] [![cat-net-badge]][cat-net]

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::Url;
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
#     errors {
#         CannotBeABase
#     }
# }

fn run() -> Result<()> {
    let full = "https://github.com/rust-lang/cargo?asdf";

    let url = Url::parse(full)?;
    let base = base_url(url)?;

    assert_eq!(base.as_str(), "https://github.com/");
    println!("The base of the URL is: {}", base);

    Ok(())
}

/// Returns the base of the given URL - the part not including any path segments
/// and query parameters.
fn base_url(mut url: Url) -> Result<Url> {
    // Clear path segments.
    match url.path_segments_mut() {
        Ok(mut path) => {
            path.clear();
        }
        Err(_) => {
            // Certain URLs cannot be turned into a base URL.
            return Err(Error::from_kind(ErrorKind::CannotBeABase));
        }
    }

    // Clear query parameters.
    url.set_query(None);

    Ok(url)
}
#
# quick_main!(run);
```

[ex-url-new-from-base]: #ex-url-new-from-base
<a name="ex-url-new-from-base"></a>
## Create new URLs from a base URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`join`] method creates a new URL from a base and relative path.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::Url;
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }

fn run() -> Result<()> {
    let path = "/rust-lang/cargo";

    let gh = build_github_url(path)?;

    assert_eq!(gh.as_str(), "https://github.com/rust-lang/cargo");
    println!("The joined URL is: {}", gh);

    Ok(())
}

fn build_github_url(path: &str) -> Result<Url> {
    // Hardcoded in our program. Caller's path will be joined to this.
    const GITHUB: &'static str = "https://github.com";

    let base = Url::parse(GITHUB).expect("hardcoded URL is known to be valid");
    let joined = base.join(path)?;

    Ok(joined)
}
#
# quick_main!(run);
```

[ex-url-origin]: #ex-url-origin
<a name="ex-url-origin"></a>
## Extract the URL origin (scheme / host / port)

[![url-badge]][url] [![cat-net-badge]][cat-net]

The [`Url`] struct exposes various methods to extract information about the URL
it represents.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::{Url, Host};

# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }
#
fn run() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    assert_eq!(url.scheme(), "ftp");
    assert_eq!(url.host(), Some(Host::Domain("rust-lang.org")));
    assert_eq!(url.port_or_known_default(), Some(21));
    println!("The origin is as expected!");

    Ok(())
}
#
# quick_main!(run);
```

The same result can be obtained using the [`origin`] method as well.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::{Url, Origin, Host};

# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }
#
fn run() -> Result<()> {
    let s = "ftp://rust-lang.org/examples";

    let url = Url::parse(s)?;

    let expected_scheme = "ftp".to_owned();
    let expected_host = Host::Domain("rust-lang.org".to_owned());
    let expected_port = 21;
    let expected = Origin::Tuple(expected_scheme, expected_host, expected_port);

    let origin = url.origin();
    assert_eq!(origin, expected);
    println!("The origin is as expected!");

    Ok(())
}
#
# quick_main!(run);
```

[ex-url-rm-frag]: #ex-url-rm-frag
<a name="ex-url-rm-frag"></a>
## Remove fragment identifiers and query pairs from a URL

[![url-badge]][url] [![cat-net-badge]][cat-net]

Parses [`Url`] and slices it with [`url::Position`] to strip unneeded URL parts.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::{Url, Position};
#
# error_chain! {
#     foreign_links {
#         UrlParse(url::ParseError);
#     }
# }

fn run() -> Result<()> {
    let parsed = Url::parse("https://github.com/rust-lang/rust/issues?labels=E-easy&state=open")?;
    let cleaned: &str = &parsed[..Position::AfterPath];
    println!("cleaned: {}", cleaned);
    Ok(())
}
#
# quick_main!(run);
```

[ex-url-basic]: #ex-url-basic
<a name="ex-url-basic"></a>
## Make a HTTP GET request

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Parses the supplied URL and makes a synchronous HTTP GET request
with [`reqwest::get`]. Prints obtained [`reqwest::Response`]
status and headers subsequently reading HTTP response body into an allocated [`String`] via [`read_to_string`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;

use std::io::Read;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         HttpRequest(reqwest::Error);
#     }
# }

fn run() -> Result<()> {
    let mut res = reqwest::get("http://httpbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());
    println!("Body:\n{}", body);

    Ok(())
}
#
# quick_main!(run);
```

[ex-url-download]: #ex-url-download
<a name="ex-url-download"></a>
## Download a file to a temporary directory

[![reqwest-badge]][reqwest] [![tempdir-badge]][tempdir] [![cat-net-badge]][cat-net] [![cat-filesystem-badge]][cat-filesystem]

Creates a temporary directory with [`TempDir::new`] and  synchronously downloads
a file over HTTP using [`reqwest::get`].
Creates a target [`File`] with name obtained from [`Response::url`] within [`TempDir::path`]
and copies downloaded data into it with [`io::copy`].
The temporary directory is automatically removed on `run` function return.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate tempdir;

use std::io::copy;
use std::fs::File;
use tempdir::TempDir;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         HttpRequest(reqwest::Error);
#     }
# }

fn run() -> Result<()> {
    // create a temp dir with prefix "example"
    let tmp_dir = TempDir::new("example")?;
    // make HTTP request for remote content
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let mut response = reqwest::get(target)?;

    let mut dest = {
        // extract target filename from URL
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.path().join(fname);
        println!("will be located under: '{:?}'", fname);
        // create file with given name inside the temp dir
        File::create(fname)?
    };
    // data is copied into the target file
    copy(&mut response, &mut dest)?;
    // tmp_dir is implicitly removed
    Ok(())
}
#
# quick_main!(run);
```

[ex-rest-get]: #ex-rest-get
<a name="ex-rest-get"/>
## Query the GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Queries GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers)
with [`reqwest::get`] to get list of all users who have marked a GitHub project with a star. [`reqwest::Response`] is deserialized with [`Response::json`] into `User` objects implementing [`serde::Deserialize`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    // remaining fields not deserialized for brevity
}
#
# error_chain! {
#     foreign_links {
#         Reqwest(reqwest::Error);
#     }
# }

fn run() -> Result<()> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let mut response = reqwest::get(&request_url)?;

    let users: Vec<User> = response.json()?;
    println!("{:?}", users);
    Ok(())
}
#
# quick_main!(run);
```

[ex-rest-head]: #ex-rest-head
<a name="ex-rest-head"/>
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

[ex-rest-custom-params]: #ex-rest-custom-params
<a name="ex-rest-custom-params"/>
## Set custom headers and URL parameters for a REST request

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![url-badge]][url] [![cat-net-badge]][cat-net]

Sets both standard and custom HTTP headers as well as URL parameters
for HTTP GET request. Firstly creates a custom header of type `XPoweredBy`
with [`hyper::header!`] macro. Secondly calls [`Url::parse_with_params`]
in order to build a complex URL with specified key value pairs.
Lastly sets standard headers [`header::UserAgent`] and [`header::Authorization`]
as well as custom one `XPoweredBy` with [`RequestBuilder::header`] prior to making
the request with [`RequestBuilder::send`].

The code is run against <http://httpbin.org/headers> service which responds with
a JSON dict containing all request headers for easy verification.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate url;
extern crate reqwest;
#[macro_use]
extern crate hyper;
#[macro_use]
extern crate serde_derive;

use std::collections::HashMap;
use url::Url;
use reqwest::Client;
use reqwest::header::{UserAgent, Authorization, Bearer};

// Custom header type
header! { (XPoweredBy, "X-Powered-By") => [String] }

// Helper for verification
#[derive(Deserialize, Debug)]
pub struct HeadersEcho {
    pub headers: HashMap<String, String>,
}
#
# error_chain! {
#     foreign_links {
#         Reqwest(reqwest::Error);
#         UrlParse(url::ParseError);
#     }
# }

fn run() -> Result<()> {
    // Make request to webservice that will respond with JSON dict containing
    // the headders set on HTTP GET request.
    let url = Url::parse_with_params("http://httpbin.org/headers",
                                     &[("lang", "rust"), ("browser", "servo")])?;

    let mut response = Client::new()
        .get(url)
        .header(UserAgent::new("Rust-test"))
        .header(Authorization(Bearer { token: "DEadBEEfc001cAFeEDEcafBAd".to_owned() }))
        .header(XPoweredBy("Guybrush Threepwood".to_owned()))
        .send()?;

    // JSON response should match the headers set on request
    let out: HeadersEcho = response.json()?;
    assert_eq!(out.headers["Authorization"],
               "Bearer DEadBEEfc001cAFeEDEcafBAd");
    assert_eq!(out.headers["User-Agent"], "Rust-test");
    assert_eq!(out.headers["X-Powered-By"], "Guybrush Threepwood");
    // Response contains full URL used to make the request
    assert_eq!(response.url().as_str(),
               "http://httpbin.org/headers?lang=rust&browser=servo");

    println!("{:?}", out);
    Ok(())
}
#
# quick_main!(run);
```

[ex-rest-post]: #ex-rest-post
<a name="ex-rest-post"/>
## Create and delete Gist with GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Creates a gist with POST request to GitHub [gists API v3](https://developer.github.com/v3/gists/) using [`Client::post`] and subsequently removes it with DELETE request using [`Client::delete`].

The [`reqwest::Client`] is responsible for details of both requests including
URL, body and authentication. POST body comes from [`serde_json::json!`] macro
which provides a way to pass an arbitrary JSON body. Call to [`RequestBuilder::json`] sets the request body while [`RequestBuilder::basic_auth`] handles authentication.
Finally the call to [`RequestBuilder::send`] synchronously executes the requests.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
use reqwest::Client;
#
# error_chain! {
#     foreign_links {
#         EnvVar(env::VarError);
#         HttpRequest(reqwest::Error);
#     }
# }

#[derive(Deserialize, Debug)]
struct Gist {
    id: String,
    html_url: String,
    // remaining fields not deserialized for brevity
}

fn run() -> Result<()> {
    let gh_user = env::var("GH_USER")?;
    let gh_pass = env::var("GH_PASS")?;

    // The type `gist_body` is `serde_json::Value`
    let gist_body = json!({
        "description": "the description for this gist",
        "public": true,
        "files": {
             "main.rs": {
             "content": r#"fn main() { println!("hello world!");}"#
            }
        }});

    // create the gist
    let request_url = "https://api.github.com/gists";
    let mut response = Client::new()
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send()?;

    let gist: Gist = response.json()?;
    println!("Created {:?}", gist);

    // delete the gist
    let request_url = format!("{}/{}",request_url, gist.id);
    let response = Client::new()
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send()?;

    println!("Gist {} deleted! Status code: {}",gist.id, response.status());
    Ok(())
}
#
# quick_main!(run);
```

For the sake of simplicity the example uses [HTTP Basic Auth] in order to
authorize access to [GitHub API]. A more typical use case would be to
employ one of the much more complex [OAuth] authorization flows.

[ex-paginated-api]: #ex-paginated-api
<a name="ex-paginated-api"></a>
## Consume a paginated RESTful API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

Wraps a paginated web API in a convenient Rust iterator. The iterator lazily
fetches the next page of results from the remote server as it arrives at the end
of each page.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
#
# error_chain! {
#     foreign_links {
#         Reqwest(reqwest::Error);
#     }
# }

#[derive(Deserialize)]
struct ApiResponse {
    dependencies: Vec<Dependency>,
    meta: Meta,
}

// Could capture more fields here if needed
#[derive(Deserialize)]
struct Dependency {
    crate_id: String,
}

#[derive(Deserialize)]
struct Meta {
    total: u32,
}

struct ReverseDependencies {
    crate_id: String,
    dependencies: <Vec<Dependency> as IntoIterator>::IntoIter,
    client: reqwest::Client,
    page: u32,
    per_page: u32,
    total: u32,
}

impl ReverseDependencies {
    fn of(crate_id: &str) -> Result<Self> {
        Ok(ReverseDependencies {
               crate_id: crate_id.to_owned(),
               dependencies: vec![].into_iter(),
               client: reqwest::Client::new(),
               page: 0,
               per_page: 100,
               total: 0,
           })
    }

    fn try_next(&mut self) -> Result<Option<Dependency>> {
        // If the previous page has a dependency that hasn't been looked at.
        if let Some(dep) = self.dependencies.next() {
            return Ok(Some(dep));
        }

        // If there are no more reverse dependencies.
        if self.page > 0 && self.page * self.per_page >= self.total {
            return Ok(None);
        }

        // Fetch the next page.
        self.page += 1;
        let url = format!("https://crates.io/api/v1/crates/{}/reverse_dependencies?page={}&per_page={}",
                          self.crate_id,
                          self.page,
                          self.per_page);

        let response = self.client.get(&url).send()?.json::<ApiResponse>()?;
        self.dependencies = response.dependencies.into_iter();
        self.total = response.meta.total;
        Ok(self.dependencies.next())
    }
}

impl Iterator for ReverseDependencies {
    type Item = Result<Dependency>;

    fn next(&mut self) -> Option<Self::Item> {
        // Some juggling required here because `try_next` returns a result
        // containing an option, while `next` is supposed to return an option
        // containing a result.
        match self.try_next() {
            Ok(Some(dep)) => Some(Ok(dep)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn run() -> Result<()> {
    for dep in ReverseDependencies::of("serde")? {
        println!("reverse dependency: {}", dep?.crate_id);
    }
    Ok(())
}
#
# quick_main!(run);
```

[ex-file-post]: #ex-file-post
<a name="ex-file-post"/>
## POST a file to paste-rs.

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

A connection is established to https://paste.rs using [`reqwest::Client`],
following the [`reqwest::RequestBuilder`] pattern.  Calling [`Client::post`]
with a URL establishes the destination, [`RequestBuilder::body`] sets the
content to send by reading the file, and [`RequestBuilder::send`] blocks until
the file uploads and the response is received.  The response is read with
[`read_to_string`], and finally displayed in the console.

```rust,no_run
extern crate reqwest;

# #[macro_use]
# extern crate error_chain;
#
use std::fs::File;
use std::io::Read;
use reqwest::Client;
#
# error_chain! {
#     foreign_links {
#         HttpRequest(reqwest::Error);
#         IoError(::std::io::Error);
#     }
# }

fn run() -> Result<()> {
    let paste_api = "https://paste.rs";
    let file = File::open("message")?;

    // blocks until paste.rs returns a response
    let mut response = Client::new().post(paste_api).body(file).send()?;
    let mut response_body = String::new();
    response.read_to_string(&mut response_body)?;
    println!("Your paste is located at: {}", response_body);
    Ok(())
}
#
# quick_main!(run);
```

[ex-random-port-tcp]: #ex-random-port-tcp
<a name="ex-random-port-tcp"></a>
## Listen on unused port TCP/IP

[![std-badge]][std] [![cat-net-badge]][cat-net]

In this example, the port is displayed on the console, and the program will
listen until a request is made.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::Read;
#
# error_chain! {
#     foreign_links {
#         Io(::std::io::Error);
#     }
# }

fn run() -> Result<()> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    // Assigning port 0 requests the OS to assign a free port
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    // read from the socket until connection closed by client, discard byte count.
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says {}", addr, input);
    Ok(())
}
#
# quick_main!(run);
```

The `std` library is leveraged to make a well formed IP/port with the
[`SocketAddrV4`] and [`Ipv4Addr`] structs.  An unused random port is requested
by passing 0 to [`TcpListener::bind`].  The assigned address is available via
[`TcpListener::local_addr`].

[`TcpListener::accept`] synchronously waits for an incoming connection and
returns a `(`[`TcpStream`],  [`SocketAddrV4`]`)` representing the request.
Reading on the socket with [`read_to_string`] will wait until the connection is
closed which can be tested with `telnet ip port`.  For example, if the program
shows Listening on 127.0.0.1:11500, run

`telnet 127.0.0.1 11500`

After sending data in telnet press `ctrl-]` and type `quit`.

[ex-extract-links-webpage]: #ex-extract-links-webpage
<a name="ex-extract-links-webpage"/>
## Extract all links from a webpage HTML

[![reqwest-badge]][reqwest] [![select-badge]][select] [![cat-net-badge]][cat-net]

Use [`reqwest::get`] to perform a HTTP GET request and then use [`Document::from_read`] to parse the response into a HTML document.
We can then retrieve all the links from the document by using [`find`] with the criteria of the [`Name`] being "a".
This returns a [`Selection`] that we [`filter_map`] on to retrieve the urls from links that have the "href" [`attr`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;
#
# error_chain! {
#    foreign_links {
#        ReqError(reqwest::Error);
#        IoError(std::io::Error);
#    }
# }

fn run() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")?;

    let document = Document::from_read(res)?;

    let links = document.find(Name("a"))
        .filter_map(|n| n.attr("href"));

    for link in links {
        println!("{}", link);
    }

    Ok(())
}
#
# quick_main!(run);
```

[ex-check-broken-links]: #ex-check-broken-links
<a name="ex-check-broken-links"/>
## Check a webpage for broken links

[![reqwest-badge]][reqwest] [![select-badge]][select] [![url-badge]][url] [![cat-net-badge]][cat-net]

We call "get_base_url" to retrieve the base URL. If the document has a "base" tag, we get the "href" [`attr`] from the first occurrence of the "base" tag. This is then used as the base URL. Otherwise, we can use [`Position::BeforePath`] with the original URL to get the base of that URL.

We iterate through all the links in the document and get the absolute URL for each (using [`url::ParseOptions`] and [`Url::parse`]). We then filter these so that we can report which links are broken.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
extern crate select;
extern crate url;

use std::collections::HashSet;

use url::{Url, Position};
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;
#
# error_chain! {
#   foreign_links {
#       ReqError(reqwest::Error);
#       IoError(std::io::Error);
#       UrlParseError(url::ParseError);
#   }
# }

fn get_base_url(url: &Url, doc: &Document) -> Result<Url> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    let base_url = base_tag_href.map_or_else(
        || Url::parse(&url[..Position::BeforePath]),
        Url::parse,
    )?;

    Ok(base_url)
}

fn check_link(url: &Url) -> Result<bool> {
    let res = reqwest::get(url.as_ref())?;

    Ok(res.status() != StatusCode::NotFound)
}

fn run() -> Result<()> {
    let url = Url::parse("https://www.rust-lang.org/en-US/")?;

    let res = reqwest::get(url.as_ref())?;
    let document = Document::from_read(res)?;

    let base_url = get_base_url(&url, &document)?;

    let base_parser = Url::options().base_url(Some(&base_url));

    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    for link in links.iter().filter(|link| check_link(link).ok() == Some(false)) {
        println!("{} is broken.", link);
    }

    Ok(())
}
#
# quick_main!(run);
```

[ex-extract-mediawiki-links]: #ex-extract-mediawiki-links
<a name="ex-extract-mediawiki-links"/>
## Extract all unique links from a MediaWiki markup

[![reqwest-badge]][reqwest] [![regex-badge]][regex] [![cat-net-badge]][cat-net]

Pull the source of a MediaWiki page using [`reqwest::get`] and then
look for all entries of internal and external links with
[`Regex::captures_iter`]. Using [`Cow`] avoids excessive [`String`] allocations.

MediaWiki link syntax is described [here][MediaWiki link syntax].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
#[macro_use]
extern crate lazy_static;
extern crate reqwest;
extern crate regex;

use std::io::Read;
use std::collections::HashSet;
use std::borrow::Cow;
use regex::Regex;

# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Reqwest(reqwest::Error);
#         Regex(regex::Error);
#     }
# }
#
fn extract_links(content: &str) -> Result<HashSet<Cow<str>>> {
    lazy_static! {
        static ref WIKI_REGEX: Regex =
            Regex::new(r"(?x)
                \[\[(?P<internal>[^\[\]|]*)[^\[\]]*\]\]    # internal links
                |
                (url=|URL\||\[)(?P<external>http.*?)[ \|}] # external links
            ").unwrap();
    }

    let links: HashSet<_> = WIKI_REGEX
        .captures_iter(content)
        .map(|c| match (c.name("internal"), c.name("external")) {
            (Some(val), None) => Cow::from(val.as_str().to_lowercase()),
            (None, Some(val)) => Cow::from(val.as_str()),
            _ => unreachable!(),
        })
        .collect();

    Ok(links)
}

fn run() -> Result<()> {
    let mut content = String::new();
    reqwest::get(
        "https://en.wikipedia.org/w/index.php?title=Rust_(programming_language)&action=raw",
    )?
        .read_to_string(&mut content)?;

    println!("{:#?}", extract_links(&content)?);

    Ok(())
}
#
# quick_main!(run);
```

[ex-progress-with-range]: #ex-progress-with-range
<a name="ex-progress-with-range"/>
## Make a partial download with HTTP range headers

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Uses [`reqwest::Client::head`] to get the content-length and validate if the server sets the header
[`reqwest::header::ContentRange`], required to confirm the support of partial downloads.

If supported, downloads the content using [`reqwest::get`], setting the [`reqwest::header::Range`]
to do partial downloads printing basic progress messages.
 in chunks of 10240 bytes

Range header is defined in [RFC7233][HTTP Range RFC7233].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;

use std::fs::File;
use reqwest::header::{ContentRange, ContentRangeSpec, Range};
use reqwest::StatusCode;
#
# error_chain! {
#     foreign_links {
#         Io(std::io::Error);
#         Reqwest(reqwest::Error);
#     }
# }
#
# struct PartialRangeIter {
#     start: u64,
#     end: u64,
#     buffer_size: u32,
# }
#
# impl PartialRangeIter {
#     pub fn new(content_range: &ContentRangeSpec, buffer_size: u32) -> Result<Self> {
#         if buffer_size == 0 {
#             Err("invalid buffer_size, give a value greater than zero.")?;
#         }
#
#         match *content_range {
#             ContentRangeSpec::Bytes { range: Some(range), .. } => Ok(PartialRangeIter {
#                 start: range.0,
#                 end: range.1,
#                 buffer_size,
#             }),
#             _ => Err("invalid range specification")?,
#         }
#     }
# }
#
# impl Iterator for PartialRangeIter {
#     type Item = Range;
#
#     fn next(&mut self) -> Option<Self::Item> {
#         if self.start > self.end {
#             None
#         } else {
#             let prev_start = self.start;
#             self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
#             Some(Range::bytes(prev_start, self.start - 1))
#         }
#     }
# }

fn run() -> Result<()> {
    // For the purpose of this example only a small download of 102400 bytes
    // with chunk size of 10240 bytes is used.
    let url = "https://httpbin.org/range/102400?duration=2";
    const CHUNK_SIZE: u32 = 10240;

    let client = reqwest::Client::new();
    let response = client.head(url).send()?;
    let range = response.headers().get::<ContentRange>().ok_or(
        "response doesn't include the expected ranges",
    )?;

    let mut output_file = File::create("download.bin")?;

    println!("starting download...");
    for range in PartialRangeIter::new(range, CHUNK_SIZE)? {

        println!("range {:?}", range);
        let mut response = client.get(url).header(range).send()?;

        let status = response.status();
        if !(status == StatusCode::Ok || status == StatusCode::PartialContent) {
            bail!("Unexpected server response: {}", status)
        }

        std::io::copy(&mut response, &mut output_file)?;
    }

    println!("Finished with success!");
    Ok(())
}
#
# quick_main!(run);
```

[ex-rate-limit-exceeded]: #ex-rate-limit-exceeded
<a name="ex-rate-limit-exceeded"/>
## Handling the Github rate limit error condition

[![reqwest-badge]][reqwest] [![hyper-badge]][hyper] [![cat-net-badge]][cat-net]

This example uses the [GitHub API - Rate limiting], as an example of how to handle error conditions.


```rust,no_run
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate hyper;
extern crate reqwest;
# extern crate rayon;

use std::time::{Duration, UNIX_EPOCH};

use reqwest::StatusCode;
# use rayon::prelude::*;

error_chain! {
    errors {
        RateLimitExceded(quote: usize, lease_secs: u64) {
            description("rate limit exceeded")
            display("rate limit exceeded the {} calls, the next call will be allowed within {} seconds", quote, lease_secs)
        }
    }
#    foreign_links {
#        Io(std::io::Error);
#        Time(std::time::SystemTimeError);
#        Reqwest(reqwest::Error);
#    }
}

header! { (XRateLimitLimit, "X-RateLimit-Limit") => [usize] }
header! { (XRateLimitRemaining, "X-RateLimit-Remaining") => [usize] }
header! { (XRateLimitReset, "X-RateLimit-Reset") => [u64] }

fn run() -> Result<()> {
    let url = "https://api.github.com/users/rust-lang-nursery ";
    let client = reqwest::Client::new();

#   let response = client.get(url).send()?;
#
#   // for the purpose of this example will exaust the limit off calls
#   if response.status() == StatusCode::Ok {
#       let rate_remaining = response.headers().get::<XRateLimitRemaining>().ok_or(
#           "response doesn't include the expected X-RateLimit-Remaining header",
#       )?;
#
#       (0..**rate_remaining).into_par_iter().for_each(|_| {
#           client.get(url).send().expect("request failed");
#       });
#   }
#
    let response = client.get(url).send()?;

    let rate_limit = response.headers().get::<XRateLimitLimit>().ok_or(
        "response doesn't include the expected X-RateLimit-Limit header",
    )?;

    let rate_remaining = response.headers().get::<XRateLimitRemaining>().ok_or(
        "response doesn't include the expected X-RateLimit-Remaining header",
    )?;

    let rate_reset_at = response.headers().get::<XRateLimitReset>().ok_or(
        "response doesn't include the expected X-RateLimit-Reset header",
    )?;

    let rate_reset_within = Duration::from_secs(**rate_reset_at) - UNIX_EPOCH.elapsed()?;

    if response.status() == StatusCode::Forbidden && **rate_remaining == 0 {
        bail!(ErrorKind::RateLimitExceded(
            **rate_limit,
            rate_reset_within.as_secs(),
        ));
    }

    println!(
        "Rate limit is currently {}/{}, the reset of this limit will be within {} seconds.",
        **rate_remaining,
        **rate_limit,
        rate_reset_within.as_secs(),
    );

    Ok(())
}
#
# quick_main!(run);
```

{{#include links.md}}

<!-- API Reference -->

[`Client::delete`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.delete
[`Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head
[`Client::post`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[`ClientBuilder::timeout`]: https://docs.rs/reqwest/*/reqwest/struct.ClientBuilder.html#method.timeout
[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`Document::from_read`]: https://docs.rs/select/*/select/document/struct.Document.html#method.from_read
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`Ipv4Addr`]: https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html
[`Name`]: https://docs.rs/select/*/select/predicate/struct.Name.html
[`Position::BeforePath`]: https://docs.rs/url/*/url/enum.Position.html#variant.BeforePath
[`Regex::captures_iter`]: https://doc.rust-lang.org/regex/regex/struct.Regex.html#method.captures_iter
[`RequestBuilder::basic_auth`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth
[`RequestBuilder::body`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.body
[`RequestBuilder::header`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.header
[`RequestBuilder::json`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.json
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`Response::json`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[`Response::url`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[`Selection`]: https://docs.rs/select/*/select/selection/struct.Selection.html
[`SocketAddrV4`]: https://doc.rust-lang.org/std/net/struct.SocketAddrV4.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`TcpListener::accept`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept
[`TcpListener::bind`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind
[`TcpListener::local_addr`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.local_addr
[`TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[`TempDir::new`]: https://docs.rs/tempdir/*/tempdir/struct.TempDir.html#method.new
[`TempDir::path`]: https://docs.rs/tempdir/*/tempdir/struct.TempDir.html#method.path
[`Url::parse_with_params`]: https://docs.rs/url/1.*/url/struct.Url.html#method.parse_with_params
[`Url`]: https://docs.rs/url/1.*/url/struct.Url.html
[`attr`]: https://docs.rs/select/*/select/node/struct.Node.html#method.attr
[`filter_map`]: https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.filter_map
[`find`]: https://docs.rs/select/*/select/document/struct.Document.html#method.find
[`header::Authorization`]: https://docs.rs/hyper/*/hyper/header/struct.Authorization.html
[`header::UserAgent`]: https://docs.rs/hyper/*/hyper/header/struct.UserAgent.html
[`hyper::header!`]: https://docs.rs/hyper/*/hyper/macro.header.html
[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
[`join`]: https://docs.rs/url/1.*/url/struct.Url.html#method.join
[`origin`]: https://docs.rs/url/1.*/url/struct.Url.html#method.origin
[`parse`]: https://docs.rs/url/1.*/url/struct.Url.html#method.parse
[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[`reqwest::RequestBuilder`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Client::head`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.head
[`reqwest::header::Range`]: https://docs.rs/reqwest/*/reqwest/header/enum.Range.html
[`reqwest::header::ContentRange`]: https://docs.rs/reqwest/*/reqwest/header/struct.ContentRange.htm
[`serde::Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
[`serde_json::json!`]: https://docs.rs/serde_json/*/serde_json/macro.json.html
[`std::iter::Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[`std::time::Duration`]: https://doc.rust-lang.org/stable/std/time/struct.Duration.html
[`std::time::UNIX_EPOCH`]: https://doc.rust-lang.org/stable/std/time/constant.UNIX_EPOCH.html
[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
[`url::Parse`]: https://docs.rs/url/*/url/struct.Url.html#method.parse
[`url::ParseOptions`]: https://docs.rs/url/*/url/struct.ParseOptions.html

<!-- Other Reference -->

[GitHub API]: https://developer.github.com/v3/auth/
[GitHub API - Rate limiting]: https://developer.github.com/v3/#rate-limiting
[HTTP Basic Auth]: https://tools.ietf.org/html/rfc2617
[MediaWiki link syntax]: https://www.mediawiki.org/wiki/Help:Links
[OAuth]: https://oauth.net/getting-started/
[HTTP Range RFC7233]: https://tools.ietf.org/html/rfc7233#section-3.1
