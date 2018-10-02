## Parse the MIME type of a HTTP response

[![reqwest-badge]][reqwest] [![mime-badge]][mime] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

When receiving a HTTP reponse from `reqwest` the [MIME type] or media type may be
found in the [Content-Type] header. [`reqwest::header::HeaderMap::get`] retrieves
the header as a [`reqwest::header::HeaderValue`], which can be converted to a
string. The `mime` crate can then parse that, yielding a [`mime::Mime`] value.

The `mime` crate also defines some commonly used MIME types.

Note that the [`reqwest::header`] module is exported from the [`http`] crate.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate mime;
extern crate reqwest;

use mime::Mime;
use std::str::FromStr;
use reqwest::header::CONTENT_TYPE;

#
# error_chain! {
#    foreign_links {
#        Reqwest(reqwest::Error);
#        Header(reqwest::header::ToStrError);
#        Mime(mime::FromStrError);
#    }
# }

fn run() -> Result<()> {
    let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png")?;
    let headers = response.headers();

    match headers.get(CONTENT_TYPE) {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(content_type) => {
            let content_type = Mime::from_str(content_type.to_str()?)?;
            let media_type = match (content_type.type_(), content_type.subtype()) {
                (mime::TEXT, mime::HTML) => "a HTML document",
                (mime::TEXT, _) => "a text document",
                (mime::IMAGE, mime::PNG) => "a PNG image",
                (mime::IMAGE, _) => "an image",
                _ => "neither text nor image",
            };

            println!("The reponse contains {}.", media_type);
        }
    };

    Ok(())
}
#
# quick_main!(run);
```

[`http`]: https://docs.rs/http/*/http/
[`mime::Mime`]: https://docs.rs/mime/*/mime/struct.Mime.html
[`reqwest::header::HeaderMap::get`]: https://docs.rs/reqwest/*/reqwest/header/struct.HeaderMap.html#method.get
[`reqwest::header::HeaderValue`]: https://docs.rs/reqwest/*/reqwest/header/struct.HeaderValue.html
[`reqwest::header`]: https://docs.rs/reqwest/*/reqwest/header/index.html

[Content-Type]: https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type
[MIME type]: https://developer.mozilla.org/docs/Web/HTTP/Basics_of_HTTP/MIME_types
