## TCP connect with a timeout

[![std-badge]][std] [![cat-net-badge]][cat-net]

Plain [`TcpStream::connect`] waits for the kernel to give up on SYN retries.
When the peer is down or a firewall is silently dropping packets, that can
take a minute or more. [`TcpStream::connect_timeout`] caps the wait.

The timeout variant takes a [`SocketAddr`], not a string, so resolve the
hostname first.

```rust,edition2021,no_run
use std::io;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

fn main() -> io::Result<()> {
    let addr = "example.com:443"
        .to_socket_addrs()?
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "no address"))?;

    match TcpStream::connect_timeout(&addr, Duration::from_secs(3)) {
        Ok(_stream) => println!("connected to {addr}"),
        Err(e) if e.kind() == io::ErrorKind::TimedOut => {
            eprintln!("connect timed out");
        }
        Err(e) => return Err(e),
    }
    Ok(())
}
```

[`TcpStream::connect`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect
[`TcpStream::connect_timeout`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect_timeout
[`SocketAddr`]: https://doc.rust-lang.org/std/net/enum.SocketAddr.html
