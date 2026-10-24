## Access a file randomly using a memory map

[![memmap2-badge]][memmap2] [![cat-filesystem-badge]][cat-filesystem]

Creates a memory map of a file using [memmap2] and simulates some non-sequential
reads from the file. Using a memory map means you just index into a slice rather
than dealing with [`seek`]ing around in a [`File`].

[`Mmap::map`] is `unsafe` because the compiler cannot guarantee that the file
behind the memory map stays unchanged. If another process modifies or truncates
the file while it is mapped, the bytes seen through the slice can change under
the program, which is a [race condition] and undefined behavior in Rust. The
caller must ensure that nothing else touches the file for as long as the map
exists, as the `SAFETY` comment in the example does.

```rust,edition2024
{{#include ../../../crates/file/memmap/src/bin/random_access.rs }}
```

[`File`]: https://doc.rust-lang.org/std/fs/struct.File.html
[`Mmap::map`]: https://docs.rs/memmap2/*/memmap2/struct.Mmap.html#method.map
[`seek`]: https://doc.rust-lang.org/std/fs/struct.File.html#method.seek
[race condition]: https://en.wikipedia.org/wiki/Race_condition#File_systems
