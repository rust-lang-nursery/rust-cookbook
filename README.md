```rust
extern crate byteorder;

use std::io::Cursor;
use byteorder::{BigEndian, ReadBytesExt};

fn main() {
   let mut rdr = Cursor::new(vec![2, 5, 3, 0]);
   // Note that we use type parameters to indicate which kind of byte
   // order we want!
   assert_eq!(517, rdr.read_u16::<BigEndian>().unwrap());
   assert_eq!(768, rdr.read_u16::<BigEndian>().unwrap());
}
```
