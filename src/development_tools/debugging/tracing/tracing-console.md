## Log messages to the console

[![tracing-badge]][tracing] [![tracing-subscriber-badge]][tracing-subscriber] [![cat-debugging-badge]][cat-debugging]

The `tracing` crate provides macros to emit log events to a tracing subscriber. The
`tracing-subscriber` crate configures where to send the events. To install the default tracing
subscriber, call [`tracing_subscriber::fmt::init()`].

[`tracing_subscriber::fmt::init()`]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/fmt/fn.init.html

```rust
use tracing::{debug, error, info, trace, warn};

fn main() {
    tracing_subscriber::fmt::init();

    error!("This is an error!");
    warn!("This is a warning.");
    info!("This is an informational message.");

    // with the default configuration, debug! and trace! messages are not shown
    debug!("This is a debug message.");
    trace!("This is a trace message.");
}
```

The default log level is `INFO`. Tracing will drop events logged at lower levels. Running this code
prints the following to the console:

<!-- 
    Generated using https://crates.io/crates/to-html, running:
    RUST_LOG=trace to-html --no-prompt  "cargo run --quiet --example=tracing-console
-->

<pre class="terminal"><code><span style='opacity:0.67'>2024-12-01T07:56:14.778440Z</span> <span style='color:var(--red,#a00)'>ERROR</span> <span style='opacity:0.67'>tracing_console:</span> This is an error!
<span style='opacity:0.67'>2024-12-01T07:56:14.778568Z</span> <span style='color:var(--yellow,#a60)'> WARN</span> <span style='opacity:0.67'>tracing_console:</span> This is a warning.
<span style='opacity:0.67'>2024-12-01T07:56:14.778596Z</span> <span style='color:var(--green,#0a0)'> INFO</span> <span style='opacity:0.67'>tracing_console:</span> This is an informational message.
</code></pre>

To configure a more verbose default level, set the `RUST_LOG` environment variable:

```shell
RUST_LOG=trace cargo run --example log-debug
```

Cargo prints these following extra lines in addition to the ones above:

<pre class="terminal"><code><span style='opacity:0.67'>2024-12-01T07:56:14.778613Z</span> <span style='color:var(--blue,#00a)'>DEBUG</span> <span style='opacity:0.67'>tracing_console:</span> This is a debug message.
<span style='opacity:0.67'>2024-12-01T07:56:14.778640Z</span> <span style='color:var(--magenta,#a0a)'>TRACE</span> <span style='opacity:0.67'>tracing_console:</span> This is a trace message.
</code></pre>
