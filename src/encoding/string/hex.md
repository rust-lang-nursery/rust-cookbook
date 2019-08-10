## hexのエンコード、デコード

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

[`data_encoding`]クレートは`HEXUPPER::encode`メソッドを提供します。これは、`&[u8]`を受け取り、データの16進数を持った文字列を返します。

同じように、`HEXUPPER::decode`メソッドは`&[u8]`を受け取り、入力データのデコードに成功したら`Vec<u8>` を返します。

下の例では`&[u8]`データを16進数と同等のデータに変換しています。この値を出力された値と比較します。

```rust
extern crate data_encoding;

use data_encoding::{HEXUPPER, DecodeError};

fn main() -> Result<(), DecodeError> {
    let original = b"The quick brown fox jumps over the lazy dog.";
    let expected = "54686520717569636B2062726F776E20666F78206A756D7073206F76\
        657220746865206C617A7920646F672E";

    let encoded = HEXUPPER.encode(original);
    assert_eq!(encoded, expected);

    let decoded = HEXUPPER.decode(&encoded.into_bytes())?;
    assert_eq!(&decoded[..], &original[..]);

    Ok(())
}
```

[`data_encoding`]: https://docs.rs/data-encoding/*/data_encoding/
