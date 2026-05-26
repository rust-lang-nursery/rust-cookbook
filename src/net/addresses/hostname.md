## Resolve a hostname to socket addresses

[![std-badge]][std] [![cat-net-badge]][cat-net]

Real config files don't carry IP addresses, they carry hostnames like
`api.internal:8080`. [`ToSocketAddrs`] turns those strings into an iterator
of [`SocketAddr`] you can connect to. It's the same trait
[`TcpStream::connect`] calls under the hood when handed a string.

A hostname can resolve to multiple addresses — at least one IPv4 and one
IPv6 on any modern host. Walk all of them when you want to display every
option, or take the first when one working address is enough.

```rust,edition2021
use std::io;
use std::net::ToSocketAddrs;

fn main() -> io::Result<()> {
    for addr in "localhost:443".to_socket_addrs()? {
        println!("localhost:443 → {addr}");
    }
    Ok(())
}
```

[`ToSocketAddrs`]: https://doc.rust-lang.org/std/net/trait.ToSocketAddrs.html
[`SocketAddr`]: https://doc.rust-lang.org/std/net/enum.SocketAddr.html
[`TcpStream::connect`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect
