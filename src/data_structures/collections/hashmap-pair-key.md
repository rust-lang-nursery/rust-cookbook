## Look Up a Tuple Key Without Cloning

[![std-badge]][std] [![cat-data-structures-badge]][cat-data-structures]

A `HashMap<(Email, Coord), V>` owns its keys, but [`HashMap::get`] only accepts `&Q` where the key type implements [`Borrow<Q>`]. A plain `&(Email, Coord)` would force the caller to build, and so clone, both halves just to ask a question. `(&Email, &Coord)` is a different type, and the standard library has no `Borrow` impl that connects it to `(Email, Coord)`.

The workaround is a small `KeyPair` trait that both tuple shapes implement. `Borrow` turns the owned tuple into a `dyn KeyPair`, and `Hash` and `Eq` for that trait object compare the two halves. The `get_pair` helper casts `&(a, b)` to `&dyn KeyPair` once, so callers just pass references and no data is copied.

The trait object's `Hash` must feed the hasher exactly what `(Email, Coord)::hash` does: the first element, then the second. If the orders differ, the lookup hashes to the wrong bucket and misses. This is the same rule as in the previous recipe, applied across a type boundary.

This technique trades complexity for fewer allocations, so it pays off mainly when key halves are heap-allocated, such as `String`, and lookups are frequent. Every hash and comparison goes through a trait object, and for cheap `Copy` keys that can be slower than cloning. Two simpler alternatives are worth trying first: nested maps, `HashMap<Email, HashMap<Coord, V>>`, which take a plain `&Email` and then a plain `&Coord` with no extra traits; and the [`Equivalent`] trait from the [`hashbrown`] crate, which lets a map be queried with any type that compares equal to the key and hashes the same way, without the `dyn` indirection.

```rust,edition2021
use std::borrow::Borrow;
use std::collections::HashMap;
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
        state.write_u8(0xff);
    }
}

trait KeyPair<A, B> {
    fn first(&self) -> &A;
    fn second(&self) -> &B;
}

impl<A, B> KeyPair<A, B> for (A, B) {
    fn first(&self) -> &A {
        &self.0
    }
    fn second(&self) -> &B {
        &self.1
    }
}

impl<A, B> KeyPair<A, B> for (&A, &B) {
    fn first(&self) -> &A {
        self.0
    }
    fn second(&self) -> &B {
        self.1
    }
}

impl<'a, A, B> Borrow<dyn KeyPair<A, B> + 'a> for (A, B)
where
    A: Eq + Hash + 'a,
    B: Eq + Hash + 'a,
{
    fn borrow(&self) -> &(dyn KeyPair<A, B> + 'a) {
        self
    }
}

impl<A: Hash, B: Hash> Hash for dyn KeyPair<A, B> + '_ {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.first().hash(state);
        self.second().hash(state);
    }
}

impl<A: PartialEq, B: PartialEq> PartialEq for dyn KeyPair<A, B> + '_ {
    fn eq(&self, other: &Self) -> bool {
        self.first() == other.first() && self.second() == other.second()
    }
}

impl<A: Eq, B: Eq> Eq for dyn KeyPair<A, B> + '_ {}

fn get_pair<'m, A, B, V>(map: &'m HashMap<(A, B), V>, a: &A, b: &B) -> Option<&'m V>
where
    A: Eq + Hash,
    B: Eq + Hash,
{
    map.get(&(a, b) as &dyn KeyPair<A, B>)
}

fn main() {
    let mut last_seen: HashMap<(Email, Coord), &str> = HashMap::new();
    last_seen.insert(
        (
            Email("ferris@example.com".to_string()),
            Coord { x: 3, y: -2 },
        ),
        "harbor",
    );

    let email = Email("FERRIS@example.com".to_string());
    let here = Coord { x: 3, y: -2 };
    let elsewhere = Coord { x: 0, y: 0 };

    let hit = get_pair(&last_seen, &email, &here);
    println!("{hit:?}");
    assert_eq!(hit, Some(&"harbor"));

    let miss = get_pair(&last_seen, &email, &elsewhere);
    println!("{miss:?}");
    assert_eq!(miss, None);
}
```

[`Borrow<Q>`]: https://doc.rust-lang.org/std/borrow/trait.Borrow.html
[`Equivalent`]: https://docs.rs/hashbrown/*/hashbrown/trait.Equivalent.html
[`HashMap::get`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get
[`hashbrown`]: https://docs.rs/hashbrown/*/hashbrown/
