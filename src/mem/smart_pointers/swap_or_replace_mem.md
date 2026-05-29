## Update text using `swap`, `take`, and `replace`

[![std-badge]][std] [![cat-rust-patterns-badge]][cat-rust-patterns]

Uses [`mem::swap`], [`mem::take`], and [`mem::replace`] to move owned String values between document fields without cloning.
[`mem::swap`] exchanges the drafts of two documents, [`mem::take`] moves a draft out for publishing while leaving an empty String behind,
and [`mem::replace`] installs the new published text while returning the previous version.

```rust,edition2021
use std::mem::{replace, swap, take};

struct Document {
    name: &'static str,
    draft: String,
    published: String,
}

impl Document {
    fn publish(&mut self) -> String {
        let next_version = take(&mut self.draft);
        replace(&mut self.published, next_version)
    }
}

fn main() {
    let mut guide = Document {
        name: "Guide",
        draft: "Guide Version 2".to_string(),
        published: "Guide Version 1".to_string(),
    };

    let mut release_notes = Document {
        name: "Release notes",
        draft: "Release notes for version 2".to_string(),
        published: "Release notes for version 1".to_string(),
    };

    swap(&mut guide.draft, &mut release_notes.draft);

    println!("Swapped:");
    println!("{} draft: {}", guide.name, guide.draft);
    println!("{} draft: {}", release_notes.name, release_notes.draft);

    let previous_version = guide.publish();

    assert!(guide.draft.is_empty());
    assert_eq!(guide.published, "Release notes for version 2");
    assert_eq!(previous_version, "Guide Version 1");
}
```

[`mem::swap`]: https://doc.rust-lang.org/std/mem/fn.swap.html
[`mem::replace`]: https://doc.rust-lang.org/std/mem/fn.replace.html
[`mem::take`]: https://doc.rust-lang.org/std/mem/fn.take.html

