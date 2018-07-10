## Get MIME type from string

[![mime-badge]][mime] [![cat-encoding-badge]][cat-encoding]

The following example shows how to parse a [`MIME`] type from a string using the
[mime] crate. [`FromStrError`] produces a default [`MIME`] type in an
`unwrap_or` clause.

```rust
extern crate mime;
use mime::{Mime, APPLICATION_OCTET_STREAM};

fn main() {
    let invalid_mime_type = "i n v a l i d";
    let default_mime = invalid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} used default value {:?}",
        invalid_mime_type, default_mime
    );

    let valid_mime_type = "TEXT/PLAIN";
    let parsed_mime = valid_mime_type
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);

    println!(
        "MIME for {:?} was parsed as {:?}",
        valid_mime_type, parsed_mime
    );
}
```

[`FromStrError`]: https://docs.rs/mime/*/mime/struct.FromStrError.html
[`MIME`]: https://docs.rs/mime/*/mime/struct.Mime.html
