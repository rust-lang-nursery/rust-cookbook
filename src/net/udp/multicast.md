## Join a UDP multicast group

[![std-badge]][std] [![cat-net-badge]][cat-net]

Multicast lets one sender reach many receivers without addressing each one
individually. Service discovery (mDNS, SSDP) and announcement protocols rely
on it. A receiver binds to the multicast port and joins a group with
[`join_multicast_v4`]; the OS then delivers any datagram sent to that group
to every receiver that joined.

Multicast group addresses live in `224.0.0.0/4`. Anything from `239.0.0.0` to
`239.255.255.255` is reserved for private use — pick from there for in-house
apps.

```rust,edition2021,no_run
use std::io;
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

fn main() -> io::Result<()> {
    let group = Ipv4Addr::new(239, 0, 0, 100);
    let port = 7878;

    // Receiver: bind to the port, then join the group on every interface.
    let receiver = UdpSocket::bind(SocketAddr::from(([0, 0, 0, 0], port)))?;
    receiver.join_multicast_v4(&group, &Ipv4Addr::UNSPECIFIED)?;

    // Sender: any local socket can send to the group address.
    let sender = UdpSocket::bind("0.0.0.0:0")?;
    sender.send_to(b"hello group", SocketAddr::new(group.into(), port))?;

    let mut buf = [0u8; 64];
    let (n, from) = receiver.recv_from(&mut buf)?;
    println!("got {n} bytes from {from}");
    Ok(())
}
```

[`join_multicast_v4`]: https://doc.rust-lang.org/std/net/struct.UdpSocket.html#method.join_multicast_v4
