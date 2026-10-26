## Define and operate on a type represented as a bitfield

[![bitflags-badge]][bitflags] [![cat-no-std-badge]][cat-no-std]

Creates a type safe bitfield type `MyFlags` with the [`bitflags!`] macro. The
macro generates the bitwise operators (`|`, `&`, `-` and `!`) along with
methods such as [`contains`], [`bits`] and [`from_bits_truncate`]. [`clear`]
comes from the [`Flags`] trait, which must be in scope. Deriving `Debug` prints
the names of the set flags.

```rust
{{#include ../../../crates/data_structures/bitfield/src/bin/bitflags.rs }}
```

### How it works

The same type can be written by hand with only the standard library. `MyFlags`
wraps a `u32`, each flag is a constant with a single bit set, and the bitwise
operators are implemented with the matching [`std::ops`] traits. This version
also implements [`Display`] to print the bits in binary.

```rust
{{#include ../../../crates/data_structures/bitfield/src/bin/handrolled.rs }}
```

[`bitflags!`]: https://docs.rs/bitflags/*/bitflags/macro.bitflags.html
[`bits`]: https://docs.rs/bitflags/*/bitflags/trait.Flags.html#tymethod.bits
[`clear`]: https://docs.rs/bitflags/*/bitflags/trait.Flags.html#method.clear
[`contains`]: https://docs.rs/bitflags/*/bitflags/trait.Flags.html#method.contains
[`Display`]: https://doc.rust-lang.org/std/fmt/trait.Display.html
[`Flags`]: https://docs.rs/bitflags/*/bitflags/trait.Flags.html
[`from_bits_truncate`]: https://docs.rs/bitflags/*/bitflags/trait.Flags.html#method.from_bits_truncate
[`std::ops`]: https://doc.rust-lang.org/std/ops/index.html
