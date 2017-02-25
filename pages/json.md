#JSON

##JSON implementation in Rust:

The example below shows two simple ways to embed JSON in Rust. 
The first method parses block JSON as a block using the parse method from the json crate. It then unwraps the parsed JSON.
The second method instantiates an object as JSON using the object macro. Key value relationships are easily set using `=>`
After demonstrating two simple ways to write JSON, the assert_eq macro ensures equivalence.

[![json][json-badge]][json]

```rust
#[macro_use]
extern crate json;

fn main(){
    let parsed_data = json::parse(r#"

    {
        "userid": 103609,
        "verified": true,
        "access_privelages": [
            "user",
            "admin"
        ]
    }

    "#).unwrap();

    let instantiated_data = object!{
        "userid" => 103609,
        "verified" => true,
        "access_privelages" => array![
            "user",
            "admin"
        ]
    };

    assert_eq!(parsed_data, instantiated_data);
}
```
# License

MIT/Apache-2.0

<!-- Links -->
[json-badge]: https://img.shields.io/crates/v/rustc-serialize.svg?label=json
[json]: http://json.rs/doc/json/

