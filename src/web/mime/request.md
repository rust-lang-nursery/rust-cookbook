## Parse the MIME type of a HTTP response

[![reqwest-badge]][reqwest] [![mime-badge]][mime] [![cat-net-badge]][cat-net] [![cat-encoding-badge]][cat-encoding]

When receiving a HTTP reponse from *reqwest* the [MIME type] or media type may be
found in the [Content-Type] header. [`reqwest::Headers::get`] retrieves the
header with the generic type [`reqwest::header::ContentType`]. Because
`ContentType` implements Deref with [`mime::Mime`] as a target, parts of the
MIME type can be obtained directly.

The *Mime* crate also has some, commonly used, predefined MIME types for
comparing and matching. *Reqwest* also exports the *mime*
crate, which can be found in the `reqwest::mime` module.

```rust,no_run
# #[macro_use]
# extern crate error_chain;
extern crate mime;
extern crate reqwest;

use reqwest::header::ContentType;
#
# error_chain! {
#    foreign_links {
#        Reqwest(reqwest::Error);
#    }
# }

fn run() -> Result<()> {
    let response = reqwest::get("https://www.rust-lang.org/logos/rust-logo-32x32.png")?;
    let headers = response.headers();

    match headers.get::<ContentType>() {
        None => {
            println!("The response does not contain a Content-Type header.");
        }
        Some(content_type) => {
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

[`mime::Mime`]: https://docs.rs/mime/*/mime/struct.Mime.html
[`reqwest::header::ContentType`]: https://docs.rs/reqwest/*/reqwest/header/struct.ContentType.html
[`reqwest::Headers::get`]: https://docs.rs/reqwest/*/reqwest/header/struct.Headers.html#method.get

[Content-Type]: https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type
[MIME type]: https://developer.mozilla.org/docs/Web/HTTP/Basics_of_HTTP/MIME_types
