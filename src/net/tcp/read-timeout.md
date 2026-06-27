## Set a read timeout on a TCP stream

[![std-badge]][std] [![cat-net-badge]][cat-net]

A default [`TcpStream`] read blocks until data arrives or the peer hangs up.
If the peer goes silent — process stuck, network partition, slow upstream —
the read hangs with it. [`set_read_timeout`] caps the wait.

On timeout the read returns an [`io::Error`]. The platform decides the kind:
[`WouldBlock`] on Unix, [`TimedOut`] on Windows. Match both or your code
breaks on the other platform.

This recipe stands up a listener that accepts but never sends, then connects
and reads with a short timeout to prove the timeout fires.

```rust,edition2021
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    // Accept the connection but never write to it.
    thread::spawn(move || {
        let _conn = listener.accept();
        thread::sleep(Duration::from_secs(60));
    });

    let mut stream = TcpStream::connect(addr)?;
    stream.set_read_timeout(Some(Duration::from_millis(50)))?;

    let mut buf = [0u8; 16];
    match stream.read(&mut buf) {
        Ok(n) => println!("received {n} bytes"),
        Err(e) if e.kind() == io::ErrorKind::WouldBlock
            || e.kind() == io::ErrorKind::TimedOut =>
        {
            println!("read timed out ({:?})", e.kind());
        }
        Err(e) => return Err(e),
    }
    Ok(())
}
```

[`TcpStream`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html
[`set_read_timeout`]: https://doc.rust-lang.org/std/net/struct.TcpStream.html#method.set_read_timeout
[`io::Error`]: https://doc.rust-lang.org/std/io/struct.Error.html
[`WouldBlock`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.WouldBlock
[`TimedOut`]: https://doc.rust-lang.org/std/io/enum.ErrorKind.html#variant.TimedOut
