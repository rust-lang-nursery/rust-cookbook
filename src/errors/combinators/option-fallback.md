## Provide fallbacks for missing values

[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

When a value may be absent, the right way to supply a default depends on how
expensive that default is to produce. [`Option::unwrap_or`] takes a value that
is *always* evaluated, so it suits cheap constants. [`Option::unwrap_or_else`]
defers to a closure, computing the fallback only when the value is `None` —
prefer it when the default is costly or has side effects.
[`Option::unwrap_or_default`] returns the type's [`Default`] value.

```rust,edition2021
fn expensive_default() -> i32 {
    // Pretend this performs costly work we want to avoid when possible.
    42
}

fn main() {
    let present: Option<i32> = Some(7);
    let missing: Option<i32> = None;

    // `unwrap_or` returns the value, or an eagerly evaluated default.
    println!("present.unwrap_or(0)       = {}", present.unwrap_or(0));
    println!("missing.unwrap_or(0)       = {}", missing.unwrap_or(0));
    assert_eq!(present.unwrap_or(0), 7);
    assert_eq!(missing.unwrap_or(0), 0);

    // `unwrap_or_else` defers the default to a closure, so the
    // fallback runs only when the value is absent.
    println!("missing.unwrap_or_else(..) = {}", missing.unwrap_or_else(expensive_default));
    assert_eq!(missing.unwrap_or_else(expensive_default), 42);

    // `unwrap_or_default` uses the type's `Default` implementation.
    println!("missing.unwrap_or_default  = {}", missing.unwrap_or_default());
    assert_eq!(missing.unwrap_or_default(), 0);
}
```

[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html
