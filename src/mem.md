# Memory Management

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Declare lazily evaluated constant][ex-lazy-constant] | [![lazy_static-badge]][lazy_static] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Std::cell][ex-std-oncecell] | [![std-badge]][std] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [`std::cell:LazyCell`][ex-std-lazycell] | [![std-badge]][std] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [`std::sync::LazyLock`][ex-std-lazylock] | [![std-badge]][std] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Store different types in one vector with `Box<dyn Trait>`][ex-std-box] | [![std-badge]][std] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Share mutable structure between owners with `Rc<RefCell<T>>`][ex-std-rc-refcell] | [![std-badge]][std] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Return borrowed or owned text with `Cow<str>`][ex-std-cow] | [![std-badge]][std] | [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Update text using `mem::swap`, `mem::take` and `mem::replace`][ex-std-mem] | [![std-badge]][std] | [![cat-rust-patterns-badge]][cat-rust-patterns] |

[ex-lazy-constant]: mem/global_static.html#declare-lazily-evaluated-constant
[ex-std-oncecell]: mem/global_static.html#stdcell
[ex-std-lazycell]: mem/global_static.html#stdcelllazycell
[ex-std-lazylock]: mem/global_static.html#stdsynclazylock
[ex-std-box]: mem/smart_pointers.html#store-different-types-in-one-vector-with-box
[ex-std-rc-refcell]: mem/smart_pointers.html#share-mutable-structure-between-owners-with-rc-and-refcell
[ex-std-cow]: mem/smart_pointers.html#return-borrowed-or-owned-text-with-cow
[ex-std-mem]: mem/smart_pointers.html#update-text-using-swap-take-and-replace

{{#include links.md}}
