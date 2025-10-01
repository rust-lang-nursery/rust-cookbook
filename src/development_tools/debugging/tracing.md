# Tracing

[`tracing`](https://crates.io/crates/tracing) is a framework for instrumenting Rust programs to
collect structured, event-based diagnostic information. It is alternative to the older
[`log`](https://crates.io/crates/log) crate and has adapters to be backwards compatible.

To enable tracing in your applicaiont, add the following crates to your project:

```shell
cargo add tracing tracing-subscriber
```

For libraries, tracing-subscriber is usually not required.

{{#include tracing/tracing-console.md}}

{{#include ../../links.md}}
