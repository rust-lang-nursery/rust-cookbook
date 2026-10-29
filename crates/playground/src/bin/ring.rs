use ring::digest::{SHA256, digest};

fn main() {
    let hash = digest(&SHA256, b"hello, playground");
    let hex: String = hash.as_ref().iter().map(|b| format!("{b:02x}")).collect();
    println!("sha256: {hex}");
}
