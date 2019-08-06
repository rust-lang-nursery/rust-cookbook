## リトルエンディアンのバイト順で整数を読み書きする

[![byteorder-badge]][byteorder] [![cat-encoding-badge]][cat-encoding]

`byteorder`構造化データの重要なバイトを反転できます。これはネットワークを介して他のシステムから情報を受信する時に必要です。

```rust
extern crate byteorder;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::Error;

#[derive(Default, PartialEq, Debug)]
struct Payload {
    kind: u8,
    value: u16,
}

fn main() -> Result<(), Error> {
    let original_payload = Payload::default();
    let encoded_bytes = encode(&original_payload)?;
    let decoded_payload = decode(&encoded_bytes)?;
    assert_eq!(original_payload, decoded_payload);
    Ok(())
}

fn encode(payload: &Payload) -> Result<Vec<u8>, Error> {
    let mut bytes = vec![];
    bytes.write_u8(payload.kind)?;
    bytes.write_u16::<LittleEndian>(payload.value)?;
    Ok(bytes)
}

fn decode(mut bytes: &[u8]) -> Result<Payload, Error> {
    let payload = Payload {
        kind: bytes.read_u8()?,
        value: bytes.read_u16::<LittleEndian>()?,
    };
    Ok(payload)
}
```
