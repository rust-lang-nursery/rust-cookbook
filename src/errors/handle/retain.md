## Avoid discarding errors during error conversions

[![thiserror-badge]][thiserror] [![cat-rust-patterns-badge]][cat-rust-patterns]

The  [thiserror] crate makes [matching] on different error types returned by
a function possible and relatively compact. [`ErrorKind`] determines the error
type.

Uses [reqwest]::[blocking] to query a random integer generator web service.  Converts
the string response into an integer. The Rust standard library,
[reqwest], and the web service can all generate errors. Well defined Rust errors
use [`foreign_links`]. An additional [`ErrorKind`] variant for the web service
error uses `errors` block of the `thiserror` derive macro.

```rust,no_run
{{#include ../../../crates/web/src/bin/retain.rs }}
```

[`ErrorKind`]: https://docs.rs/thiserror/*/thiserror/
[`foreign_links`]: https://docs.rs/thiserror/*/thiserror/#foreign-links
[blocking]: https://docs.rs/reqwest/*/reqwest/blocking/index.html
[Matching]:https://docs.rs/thiserror/*/thiserror/#matching-errors
