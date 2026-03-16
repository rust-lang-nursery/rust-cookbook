# Memory Management

| Recipe | Crates | Categories |
|--------|--------|------------|
| [Declare lazily evaluated constant][ex-lazy-constant] | [![lazy_static-badge]][lazy_static] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Std::cell][ex-std-oncecell] | [![std-badge]][std] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [`std::cell:LazyCell`][ex-std-lazycell] | [![std-badge]][std] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [`std::sync::LazyLock`][ex-std-lazylock] | [![std-badge]][std] | [![cat-caching-badge]][cat-caching] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Actor Pattern with Tokio][ex-actor-pattern] | [![tokio-badge]][tokio] | [![cat-concurrency-badge]][cat-concurrency] [![cat-rust-patterns-badge]][cat-rust-patterns] |
| [Custom Future (Pin, Waker, Poll)][ex-custom-future] | [![std-badge]][std] | [![cat-concurrency-badge]][cat-concurrency] [![cat-rust-patterns-badge]][cat-rust-patterns] |

[ex-lazy-constant]: mem/global_static.html#declare-lazily-evaluated-constant
[ex-std-oncecell]: mem/global_static.html#stdcell
[ex-std-lazycell]: mem/global_static.html#stdcelllazycell
[ex-std-lazylock]: mem/global_static.html#stdsynclazylock
[ex-actor-pattern]: mem/actor.html#actor-pattern-with-tokio-handleactormessage
[ex-custom-future]: mem/custom_future.html#implement-a-custom-future-pin-waker-poll


{{#include links.md}}
