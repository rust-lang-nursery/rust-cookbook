## 文字列をapplication/x-www-form-urlencodedとしてエンコードする

[![url-badge]][url] [![cat-encoding-badge]][cat-encoding]

[`form_urlencoded::byte_serialize`]で文字列を[application/x-www-form-urlencoded]構文に変換して、[`form_urlencoded::parse`]でデコードします。どちらの関数も`String`になるイテレータを返します。

```rust
extern crate url;
use url::form_urlencoded::{byte_serialize, parse};

fn main() {
    let urlencoded: String = byte_serialize("What is ❤?".as_bytes()).collect();
    assert_eq!(urlencoded, "What+is+%E2%9D%A4%3F");
    println!("urlencoded:'{}'", urlencoded);

    let decoded: String = parse(urlencoded.as_bytes())
        .map(|(key, val)| [key, val].concat())
        .collect();
    assert_eq!(decoded, "What is ❤?");
    println!("decoded:'{}'", decoded);
}
```

[`form_urlencoded::byte_serialize`]: https://docs.rs/url/*/url/form_urlencoded/fn.byte_serialize.html
[`form_urlencoded::parse`]: https://docs.rs/url/*/url/form_urlencoded/fn.parse.html

[application/x-www-form-urlencoded]: https://url.spec.whatwg.org/#application/x-www-form-urlencoded
