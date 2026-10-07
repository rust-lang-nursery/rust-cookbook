## Use a Custom Type as a HashMap Key

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

A type can be a [`HashMap`] key or [`HashSet`] member when it implements [`Eq`] and [`Hash`]. When equality is plain field-by-field comparison, `#[derive(Hash, PartialEq, Eq)]` is enough, as with `Coord` below.

A manual [`Hash`] implementation is needed when equality is custom. `Email` treats addresses as equal regardless of letter case, so its `Hash` must also ignore case: it feeds the lowercased bytes to the hasher. The rule is that `a == b` must imply `hash(a) == hash(b)`; if the two disagree, equal keys can land in different buckets and lookups silently miss. The assertions at the end check this property.

```rust,edition2021
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Debug, Hash, PartialEq, Eq)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Email(String);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}

impl Eq for Email {}

impl Hash for Email {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for byte in self.0.bytes() {
            state.write_u8(byte.to_ascii_lowercase());
        }
        // Mirrors what `str::hash` does, so a prefix cannot collide with a longer value.
        state.write_u8(0xff);
    }
}

fn hash_of<T: Hash>(value: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    value.hash(&mut hasher);
    hasher.finish()
}

fn main() {
    let mut grid: HashMap<Coord, &str> = HashMap::new();
    grid.insert(Coord { x: 0, y: 0 }, "origin");
    grid.insert(Coord { x: 3, y: -2 }, "treasure");
    println!("{:?}", grid.get(&Coord { x: 3, y: -2 }));
    assert_eq!(grid.get(&Coord { x: 3, y: -2 }), Some(&"treasure"));

    let mut seen: HashSet<Email> = HashSet::new();
    seen.insert(Email("Ferris@Example.com".to_string()));
    let is_new = seen.insert(Email("ferris@example.COM".to_string()));
    println!("second insert was new: {is_new}");
    assert!(!is_new);
    assert_eq!(seen.len(), 1);

    let a = Email("Ferris@Example.com".to_string());
    let b = Email("FERRIS@example.com".to_string());
    assert_eq!(a, b);
    assert_eq!(hash_of(&a), hash_of(&b));
}
```

[`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html
[`HashMap`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`HashSet`]: https://doc.rust-lang.org/std/collections/struct.HashSet.html
