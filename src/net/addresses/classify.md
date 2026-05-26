## Classify an IP address

[![std-badge]][std] [![cat-net-badge]][cat-net]

Before connecting to an address pulled from user input, a config file, or an
HTTP redirect, check what kind it is. Loopback, link-local, and RFC1918
private ranges have no business being targets for outbound traffic from a
server exposed on the public internet — letting them through is the SSRF
vector.

[`IpAddr`] exposes [`is_loopback`], [`is_multicast`], and [`is_unspecified`]
on both v4 and v6. [`is_private`] and [`is_link_local`] are on [`Ipv4Addr`]
only — match the variant to reach them.

```rust,edition2021
use std::net::IpAddr;
use std::str::FromStr;

fn classify(addr: IpAddr) -> &'static str {
    if addr.is_loopback() { return "loopback"; }
    if addr.is_multicast() { return "multicast"; }
    if addr.is_unspecified() { return "unspecified"; }
    if let IpAddr::V4(v4) = addr {
        if v4.is_private() { return "private (RFC1918)"; }
        if v4.is_link_local() { return "link-local"; }
    }
    "public"
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let samples = ["127.0.0.1", "10.0.0.5", "169.254.1.1", "8.8.8.8", "224.0.0.1", "::1"];
    for s in samples {
        let addr = IpAddr::from_str(s)?;
        println!("{addr}: {}", classify(addr));
    }
    Ok(())
}
```

[`IpAddr`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html
[`Ipv4Addr`]: https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html
[`is_loopback`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html#method.is_loopback
[`is_multicast`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html#method.is_multicast
[`is_unspecified`]: https://doc.rust-lang.org/std/net/enum.IpAddr.html#method.is_unspecified
[`is_private`]: https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html#method.is_private
[`is_link_local`]: https://doc.rust-lang.org/std/net/struct.Ipv4Addr.html#method.is_link_local
