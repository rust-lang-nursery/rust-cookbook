## (De)-Serialize a Matrix
[![ndarray-badge]][ndarray] [![cat-science-badge]][cat-science]

Serialize and deserialize a matrix to and from JSON. Serialization is taken care of
by [`serde_json::to_string`] and [`serde_json::from_str`] performs deserialization.

Note that serialization followed by deserialization gives back the original matrix.

```rust
extern crate nalgebra;
extern crate serde_json;

use nalgebra::DMatrix;

fn main() -> Result<(), std::io::Error> {
    let row_slice: Vec<i32> = (1..5001).collect();
    let matrix = DMatrix::from_row_slice(50, 100, &row_slice);

    // serialize matrix
    let serialized_matrix = serde_json::to_string(&matrix)?;

    // deserialize matrix
    let deserialized_matrix: DMatrix<i32> = serde_json::from_str(&serialized_matrix)?;

    // verify that `deserialized_matrix` is equal to `matrix`
    assert!(deserialized_matrix == matrix);

    Ok(())
}
```

[`serde_json::to_string`]: https://docs.rs/serde_json/*/serde_json/fn.to_string.html
[`serde_json::from_str`]: https://docs.rs/serde_json/*/serde_json/fn.from_str.html
