## Return borrowed or owned text with `Cow`
[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

Uses [`borrow::Cow`] to avoid allocating when the input string already has the desired form. The function returns [`Cow::Borrowed`] for unchanged input and [`Cow::Owned`]
only when normalization requires a new [`String`], such as converting uppercase text with [`str::to_lowercase`]. 
This keeps the fast path allocation-free while still allowing transformed output.

```rust,edition2021
use std::borrow::Cow;

fn normalize(input: &str) -> Cow<'_, str> {
    if input.chars().any(|c| c.is_uppercase()) {
        Cow::Owned(input.to_lowercase())
    } else {
        Cow::Borrowed(input)
    }
}

fn main() {
    let owned = normalize("Changes are REQUIRED, this will be Cow::Owned");
    let borrowed = normalize("no changes required, this will be cow::borrowed");

    assert!(matches!(owned, Cow::Owned(_)));
    assert!(matches!(borrowed, Cow::Borrowed(_)));

    println!("{}", owned);
    println!("{}", borrowed);
}
```
[`str::to_lowercase`]: https://doc.rust-lang.org/std/primitive.str.html#method.to_lowercase
[`borrow::Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`Cow::Borrowed`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Borrowed
[`Cow::Owned`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Owned
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
