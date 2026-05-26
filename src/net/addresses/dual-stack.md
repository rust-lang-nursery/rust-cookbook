## Bind for both IPv4 and IPv6

[![std-badge]][std] [![cat-net-badge]][cat-net]

`0.0.0.0` accepts IPv4 clients only. To accept both protocols with a single
listener, bind to the unspecified IPv6 address `[::]`. On Linux and macOS the
listener accepts IPv6 clients natively and IPv4 clients via IPv4-mapped
addresses (`::ffff:1.2.3.4`).

On Windows and FreeBSD the `IPV6_V6ONLY` socket option defaults to true, so
the same bind accepts IPv6 only. [`std::net`] doesn't expose `IPV6_V6ONLY`;
the [`socket2`] crate does, or you can open two listeners.

```rust,edition2021
use std::io;
use std::net::TcpListener;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("[::]:0")?;
    let addr = listener.local_addr()?;
    println!("listening on {addr}");
    Ok(())
}
```

[`std::net`]: https://doc.rust-lang.org/std/net/index.html
[`socket2`]: https://docs.rs/socket2
