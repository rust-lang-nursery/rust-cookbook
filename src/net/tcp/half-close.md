## Half-close a TCP connection

[![std-badge]][std] [![cat-net-badge]][cat-net]

Many request-then-response protocols expect the client to finish sending
before the server replies: HTTP/1.0, custom RPCs, anything that reads
until EOF. To say "I'm done sending, give me your reply" without losing the
ability to read, shut down only the write side with [`Shutdown::Write`].

After the half-close the peer sees EOF on its read, can do its work, and
writes a response that you read normally. [`read_to_end`] keeps reading until
the peer closes its own write half.

```rust,edition2021,no_run
use std::io::{self, Read, Write};
use std::net::{Shutdown, TcpStream};

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("example.com:80")?;
    stream.write_all(b"GET / HTTP/1.0\r\nHost: example.com\r\n\r\n")?;
    stream.shutdown(Shutdown::Write)?;

    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;
    println!("received {} bytes", response.len());
    Ok(())
}
```

[`Shutdown::Write`]: https://doc.rust-lang.org/std/net/enum.Shutdown.html#variant.Write
[`read_to_end`]: https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_end
