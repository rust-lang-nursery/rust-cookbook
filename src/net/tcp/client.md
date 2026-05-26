## TCP client

[![std-badge]][std] [![cat-net-badge]][cat-net]

With the echo server from the previous recipe running on `127.0.0.1:7878`, a
client can connect, send a message, and read the reply.

[`TcpStream::connect`] opens a connection. The returned [`TcpStream`] is both a
reader and a writer. [`write_all`] sends the request; [`read`] fills a buffer
with whatever the server returns. Echo guarantees the bytes round-trip
unchanged, so the client can assert on what came back.

```rust,edition2021,no_run
use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    stream.write_all(b"ping")?;

    let mut buf = [0u8; 4];
    stream.read_exact(&mut buf)?;
    println!("echo: {}", std::str::from_utf8(&buf).unwrap_or("<binary>"));

    assert_eq!(&buf, b"ping");
    Ok(())
}
```

[`TcpStream::connect`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.connect
[`TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[`write_all`]: https://doc.rust-lang.org/std/io/trait.Write.html#method.write_all
[`read`]: https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
