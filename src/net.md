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
| [Check if an API Resource Exists][ex-rest-head] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Create and delete Gist with GitHub API][ex-rest-post] | [![reqwest-badge]][reqwest] [![serde-badge]][serde] | [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding] |
| [POST a file to paste-rs][ex-file-post] | [![reqwest-badge]][reqwest] | [![cat-net-badge]][cat-net] |
| [Listen on Unused port TCP/IP][ex-random-port-tcp] | [![std-badge]][std] | [![cat-net-badge]][cat-net] |

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
#    foreign_links {
#        UrlParse(url::ParseError);
#    }
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
#    foreign_links {
#        UrlParse(url::ParseError);
#    }
#    errors {
#        CannotBeABase
#    }
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
#    foreign_links {
#        UrlParse(url::ParseError);
#    }
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
#    foreign_links {
#        UrlParse(url::ParseError);
#    }
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
#    foreign_links {
#        UrlParse(url::ParseError);
#    }
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

Once [`Url`] is parsed it can be sliced with [`url::Position`] to strip unneeded URL parts.

```rust
# #[macro_use]
# extern crate error_chain;
extern crate url;

use url::{Url, Position};
#
# error_chain! {
#    foreign_links {
#        UrlParse(url::ParseError);
#    }
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

The [`reqwest::get`] function parses the supplied url and makes a
synchronous HTTP GET request. Obtained [`reqwest::Response`]
status and headers are printed. HTTP response body is read into an allocated [`String`] via [`read_to_string`].

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;

use std::io::Read;
#
# error_chain! {
#    foreign_links {
#        Io(std::io::Error);
#        HttpReqest(reqwest::Error);
#    }
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

Temporary directory is created with [`TempDir::new`] and a file is synchronously
downloaded over HTTP using [`reqwest::get`].
Target [`File`] with name obtained from [`Response::url`] is created within [`TempDir::path`]
and downloaded data is copied into it with [`io::copy`]. The temporary directory is implicitly removed on `run` function return.

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
#    foreign_links {
#        Io(std::io::Error);
#        HttpReqest(reqwest::Error);
#    }
# }

fn run() -> Result<()> {
    // create a temp dir with prefix "example"
    let tmp_dir = TempDir::new("example")?;
    // make HTTP request for remote content
    let target = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let mut response = reqwest::get(target)?;

    let mut dest = {
        // extract target filename from URL
        let fname = response.url()
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

GitHub [stargazers API v3](https://developer.github.com/v3/activity/starring/#list-stargazers) is queried with [`reqwest::get`] to obtain list of all users who have marked a GitHub project with a star. [`reqwest::Response`] is deserialized with [`Response::json`] into `User` objects implementing [`serde::Deserialize`].

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
#    foreign_links {
#        Reqwest(reqwest::Error);
#    }
# }

fn run() -> Result<()> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "brson",
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
## Check if a resource exists using a HEAD request

[![reqwest-badge]][reqwest] [![cat-net-badge]][cat-net]

Query the [GitHub Users Endpoint](https://api.github.com/users) using a HEAD request and then inspect the
response code to determine success. This is a quick way to query a rest resource without
needing to receive a body. You can also configure the [`reqwest::Client`] to use a timeout
which ensures that a request will not last longer than what is passed to the timeout function.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;

use std::time::Duration;
use reqwest::Client;
#
# error_chain! {
#    foreign_links {
#        Reqwest(reqwest::Error);
#    }
# }

fn run() -> Result<()> {
    let request_url = "https://api.github.com/users/ferris-the-crab";
    println!("{}", request_url);

    // The timeout for the request is set to 5 seconds.
    let timeout = Duration::new(5, 0);

    let mut client = Client::new()?;
    client.timeout(timeout);
    let response = client.head(request_url).send()?;

    if response.status().is_success() {
        println!("ferris-the-crab is a user!");
    } else {
        println!("ferris-the-crab is not a user!");
    }

    Ok(())
}
#
# quick_main!(run);
```

[ex-rest-post]: #ex-rest-post
<a name="ex-rest-post"/>
## Create and delete Gist with GitHub API

[![reqwest-badge]][reqwest] [![serde-badge]][serde] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

HTTP POST request to [gists API v3](https://developer.github.com/v3/gists/) is made with [`reqwest::Client`] in order to create a gist.
A request body is created with [`serde_json::json!`] macro and
set set with [`RequestBuilder::json`].
Request is prepared with [`Client::post`], authenticated with [`RequestBuilder::basic_auth`] and synchronously executed with [`RequestBuilder::send`].

Gist is subsequently deleted with HTTP DELETE request prepared with [`Client::delete`] and executed as before.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate reqwest;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::env;
#
# error_chain! {
#   foreign_links {
#       EnvVar(env::VarError);
#       HttpReqest(reqwest::Error);
#   }
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
    let client = reqwest::Client::new()?;
    let mut response = client
        .post(request_url)
        .basic_auth(gh_user.clone(), Some(gh_pass.clone()))
        .json(&gist_body)
        .send()?;

    let gist: Gist = response.json()?;
    println!("Created {:?}", gist);

    // delete the gist
    let request_url = format!("{}/{}",request_url, gist.id);
    let client = reqwest::Client::new()?;
    let response = client
        .delete(&request_url)
        .basic_auth(gh_user, Some(gh_pass))
        .send()?;

    println!("Gist {} deleted! Status code: {}",gist.id, response.status());
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

```rust no_run
extern crate reqwest;

# #[macro_use]
# extern crate error_chain;
#
use std::fs::File;
use std::io::Read;
use reqwest::Client;
#
# error_chain! {
#    foreign_links {
#        HttpReqest(reqwest::Error);
#        IoError(::std::io::Error);
#    }
# }

fn run() -> Result<()> {
    let paste_api = "https://paste.rs";
    let file = File::open("message")?;
    let client = Client::new()?;

    // blocks until paste.rs returns a response
    let mut response = client
        .post(paste_api)
        .body(file)
        .send()?;
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
## Listen on Unused port TCP/IP

[![std-badge]][std] [![cat-net-badge]][cat-net]

In this example, the port is displayed on the console, and the program will
listen until a request is made.  

```rust, no_run
# #[macro_use]
# extern crate error_chain;
#
use std::net::{SocketAddrV4, Ipv4Addr, TcpListener};
use std::io::Read;

# error_chain! {
#    foreign_links {
#        Io(::std::io::Error);
#    }
# }
#
fn run() -> Result<()> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    // Assigning port 0 requests the OS to assign a free port
    let socket = SocketAddrV4::new(loopback, 0);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;
    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?;  //block  until requested
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
<!-- Categories -->

[cat-encoding-badge]: https://img.shields.io/badge/-encoding-red.svg
[cat-encoding]: https://crates.io/categories/encoding
[cat-filesystem-badge]: https://img.shields.io/badge/-filesystem-red.svg
[cat-filesystem]: https://crates.io/categories/filesystem
[cat-net-badge]: https://img.shields.io/badge/-net-red.svg
[cat-net]: https://crates.io/categories/network-programming

<!-- Crates -->

[reqwest-badge]: https://img.shields.io/crates/v/reqwest.svg?label=reqwest
[reqwest]: https://docs.rs/reqwest/
[serde-badge]: https://img.shields.io/crates/v/serde.svg?label=serde
[serde]: https://docs.rs/serde/
[std]: https://doc.rust-lang.org/std
[std-badge]: https://img.shields.io/badge/std-1.17.0-blue.svg
[tempdir-badge]: https://img.shields.io/crates/v/tempdir.svg?label=tempdir
[tempdir]: https://docs.rs/tempdir/
[url-badge]: https://img.shields.io/crates/v/url.svg?label=url
[url]: https://docs.rs/url/

<!-- Reference -->

[`io::copy`]: https://doc.rust-lang.org/std/io/fn.copy.html
[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`Url`]: https://docs.rs/url/1.*/url/struct.Url.html
[`parse`]: https://docs.rs/url/1.*/url/struct.Url.html#method.parse
[`url::Position`]: https://docs.rs/url/*/url/enum.Position.html
[`origin`]: https://docs.rs/url/1.*/url/struct.Url.html#method.origin
[`join`]: https://docs.rs/url/1.*/url/struct.Url.html#method.join
[`reqwest::get`]: https://docs.rs/reqwest/*/reqwest/fn.get.html
[`reqwest::Client`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html
[`reqwest::Response`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html
[`Response::url`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.url
[`Response::json`]: https://docs.rs/reqwest/*/reqwest/struct.Response.html#method.json
[`RequestBuilder::basic_auth`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.basic_auth
[`Client::delete`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.delete
[`Client::post`]: https://docs.rs/reqwest/*/reqwest/struct.Client.html#method.post
[`RequestBuilder::body`]: https://docs.rs/reqwest/0.6.2/reqwest/struct.RequestBuilder.html#method.body
[`RequestBuilder::json`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.json
[`RequestBuilder::send`]: https://docs.rs/reqwest/*/reqwest/struct.RequestBuilder.html#method.send
[`read_to_string`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`serde::Deserialize`]: https://docs.rs/serde/*/serde/trait.Deserialize.html
[`serde_json::json!`]: https://docs.rs/serde_json/*/serde_json/macro.json.html
[`TempDir::new`]: https://docs.rs/tempdir/*/tempdir/struct.TempDir.html#method.new
[`TempDir::path`]: https://docs.rs/tempdir/*/tempdir/struct.TempDir.html#method.path
[`reqwest::RequestBuilder`]: https://docs.rs/reqwest/0.6.2/reqwest/struct.RequestBuilder.html
[`Ipv4Addr`]: https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html
[`SocketAddrV4`]: https://doc.rust-lang.org/std/net/struct.SocketAddrV4.html
[`TcpListener::accept`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept
[`TcpListener::bind`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind
[`TcpListener::local_addr`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.local_addr
[`TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
