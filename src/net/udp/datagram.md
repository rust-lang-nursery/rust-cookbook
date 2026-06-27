## Send and receive a UDP datagram

[![std-badge]][std] [![cat-net-badge]][cat-net]

UDP is connectionless: the sender fires a packet at an address and the receiver
picks it up if it arrives. There is no session, no handshake, no retransmit.
Statsd, syslog, NTP, and DNS all run on UDP for the same reason — minimum
overhead when delivery guarantees aren't worth the cost.

This recipe wires both ends together so it runs as a single program. The
collector binds to an OS-allocated port and waits with [`recv_from`]. The
client binds to its own ephemeral port and uses [`send_to`] to deliver a
metric. [`recv_from`] returns the byte count and the sender's address.

```rust,edition2021
use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    let collector = UdpSocket::bind("127.0.0.1:0")?;
    let collector_addr = collector.local_addr()?;

    let client = UdpSocket::bind("127.0.0.1:0")?;
    client.send_to(b"app.requests:1|c", collector_addr)?;

    let mut buf = [0u8; 1500];
    let (n, from) = collector.recv_from(&mut buf)?;
    let metric = std::str::from_utf8(&buf[..n]).unwrap_or("<binary>");
    println!("received {n} bytes from {from}: {metric}");
    Ok(())
}
```

[`recv_from`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.recv_from
[`send_to`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.send_to
