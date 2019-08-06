## 構造化されていないJSONのシリアライズ、デシリアライズ

[![serde-json-badge]][serde-json] [![cat-encoding-badge]][cat-encoding]

[`serde_json`]クレートはJSONの`&str`をパースするための[`from_str`]関数を提供します。

非構造化JSONはどんなvalidのJSONデータでも使える共通な[`serde_json::Value`]型にパースすることができます。

この例ではJSONの`&str`をパースしています。導かれた値は[`json!`]マクロで宣言されています
```rust
#[macro_use]
extern crate serde_json;

use serde_json::{Value, Error};

fn main() -> Result<(), Error> {
    let j = r#"{
                 "userid": 103609,
                 "verified": true,
                 "access_privileges": [
                   "user",
                   "admin"
                 ]
               }"#;

    let parsed: Value = serde_json::from_str(j)?;

    let expected = json!({
        "userid": 103609,
        "verified": true,
        "access_privileges": [
            "user",
            "admin"
        ]
    });

    assert_eq!(parsed, expected);

    Ok(())
}
```

[`from_str`]: https://docs.serde.rs/serde_json/fn.from_str.html
[`json!`]: https://docs.serde.rs/serde_json/macro.json.html
[`serde_json`]: https://docs.serde.rs/serde_json/
[`serde_json::Value`]: https://docs.serde.rs/serde_json/enum.Value.html
