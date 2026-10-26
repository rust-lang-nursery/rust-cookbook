## Generate OTP with a Base32 set

[![data-encoding-badge]][data-encoding] [![cat-encoding-badge]][cat-encoding]

The [`data_encoding`] crate provides a `BASE32_NOPAD::encode` method which takes
a `&[u8]` and returns a `String` containing the base32 representation of the
data.

Similarly, a `BASE32_NOPAD::decode` method is provided which takes a `&[u8]` and
returns a `Vec<u8>` if the input data is successfully decoded. There is also a
`BASE32_NOPAD_VISUAL::decode` (resp. `BASE32_NOPAD_NOCASE::decode`) method which
accepts characters outside base32 that can easily be confused with characters
inside base32 and understands them accordingly (resp. accepts lowercase
characters and understands them as their uppercase version).

The recipe below generates a human-readable one-time password by converting
random bytes to base32. It then assumes the password was entered by a human with
some visual errors. It finally makes sure the entered password decodes back to
the initial random bytes.

```rust
{{#include ../../../crates/encoding/base32/src/bin/base32.rs }}
```

[`data_encoding`]: https://docs.rs/data-encoding/*/data_encoding/
