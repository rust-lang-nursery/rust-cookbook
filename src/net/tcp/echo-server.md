## TCP echo server

[![std-badge]][std] [![cat-net-badge]][cat-net]

When you're building a TCP client you need a server that behaves predictably.
An echo server is that server: every byte you send comes back unchanged. Point
your client at it to check framing, disconnect handling, or timeouts.

[`TcpListener::bind`] starts listening on an address. [`incoming`] yields each
new connection. Spawning a thread per connection keeps the accept loop free for
the next client. Inside the handler, [`read`] returning `0` means the peer
closed the connection — that's the cue to drop out of the loop.

```rust,edition2021,no_run
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0u8; 1024];
    loop {
        let n = stream.read(&mut buf)?;
        if n == 0 {
            return Ok(());
        }
        stream.write_all(&buf[..n])?;
    }
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("echo server listening on {}", listener.local_addr()?);
    for incoming in listener.incoming() {
        let stream = incoming?;
        thread::spawn(move || {
            if let Err(e) = handle_connection(stream) {
                eprintln!("connection error: {e}");
            }
        });
    }
    Ok(())
}
```

[`TcpListener::bind`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.bind
[`incoming`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.incoming
[`read`]: https://doc.rust-lang.org/std/io/trait.Read.html#tymethod.read
