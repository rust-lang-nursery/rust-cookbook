## Group Records by Key

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

Groups a flat list of records into a [`HashMap`] of vectors using [`entry`] with [`or_default`], which inserts an empty `Vec` the first time a key is seen and returns a mutable reference to it on every call. [`keys`] with [`copied`] produces an owned `Vec<&str>` for sorting — `copied` turns the `&&str` references that `keys` yields into `&str` values.

```rust,edition2021
use std::collections::HashMap;

fn main() {
    let log_lines = vec![
        ("ERROR", "disk full"),
        ("WARN",  "high memory usage"),
        ("INFO",  "server started"),
        ("ERROR", "connection refused"),
        ("INFO",  "request received"),
    ];

    let mut by_level: HashMap<&str, Vec<&str>> = HashMap::new();
    for (level, message) in log_lines {
        by_level.entry(level).or_default().push(message);
    }

    let mut levels: Vec<&str> = by_level.keys().copied().collect();
    levels.sort();
    for level in levels {
        println!("[{level}]");
        for msg in &by_level[level] {
            println!("  {msg}");
        }
    }

    assert_eq!(by_level["ERROR"].len(), 2);
    assert_eq!(by_level["INFO"].len(), 2);
    assert_eq!(by_level["WARN"].len(), 1);
}
```

[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`entry`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.entry
[`or_default`]: https://doc.rust-lang.org/std/collections/hash_map/enum.Entry.html#method.or_default
[`keys`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.keys
[`copied`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.copied
