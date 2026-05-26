## Disable Nagle's algorithm

[![std-badge]][std] [![cat-net-badge]][cat-net]

TCP coalesces small writes by default — Nagle's algorithm — to avoid
flooding the network with tiny packets. For interactive protocols that's the
wrong tradeoff: a typed keystroke or a chat message can sit in the kernel
buffer for tens of milliseconds waiting for company. [`set_nodelay`] turns
Nagle off so each write goes out as its own segment.

Use it for chat, games, REPL-style protocols, and RPCs where round-trip
latency matters more than packet count. Leave it on for bulk transfer.

```rust,edition2021,no_run
use std::io::{self, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.set_nodelay(true)?;

    stream.write_all(b"ping ")?;
    stream.write_all(b"pong\n")?;
    Ok(())
}
```

[`set_nodelay`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_nodelay
