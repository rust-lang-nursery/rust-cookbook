## Non-blocking TCP accept

[![std-badge]][std] [![cat-net-badge]][cat-net]

A normal [`accept`] blocks until a client arrives. If a single thread needs
to do other work between connections — poll a timer, watch another file
descriptor — flip the listener into non-blocking mode with
[`set_nonblocking`]. [`accept`] then returns immediately with an
[`io::Error`] of kind [`WouldBlock`] when nothing is pending.

This readiness pattern is the primitive every async runtime uses underneath.
For production code, reach for [`mio`] or an async runtime — what follows is
the manual version of what they automate.

```rust,edition2021
use std::io::{self, ErrorKind};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    listener.set_nonblocking(true)?;

    // Spawn a client so the listener has something to accept.
    thread::spawn(move || {
        let _ = TcpStream::connect(addr);
    });

    let stream = loop {
        match listener.accept() {
            Ok((s, _)) => break s,
            Err(e) if e.kind() == ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => return Err(e),
        }
    };

    println!("accepted from {}", stream.peer_addr()?);
    Ok(())
}
```

[`accept`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.accept
[`set_nonblocking`]: https://doc.rust-lang.org/std/net/struct.TcpListener.html#method.set_nonblocking
[`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`WouldBlock`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.WouldBlock
[`mio`]: https://docs.rs/mio
