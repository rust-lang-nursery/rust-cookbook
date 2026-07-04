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

```rust,edition2018
use data_encoding::{BASE32_NOPAD, BASE32_NOPAD_VISUAL, DecodeError};

fn main() -> Result<(), DecodeError> {
    // Generate the password from random bytes.
    let initial_entropy = rand::random::<[u8; 10]>(); // 80 bits of entropy
    let shown_password = make_password(&initial_entropy);

    // Show the password to the user.
    println!("{shown_password}");

    // Read the password from the user (may have visual mistakes).
    let entered_password = alter_visually(&shown_password);
    println!("{entered_password}");

    // Reconstitute the random bytes (correcting the visual errors).
    let final_entropy = read_entropy(&entered_password)?;
    assert_eq!(final_entropy, initial_entropy);

    Ok(())
}

fn make_password(entropy: &[u8]) -> String {
    BASE32_NOPAD.encode(entropy)
}

fn read_entropy(password: &str) -> Result<Vec<u8>, DecodeError> {
    BASE32_NOPAD_VISUAL.decode(password.as_bytes())
}

fn alter_visually(input: &str) -> String {
    fn alter_char(c: char) -> char {
        match c {
            // O may be confused with 0
            'O' if rand::random::<bool>() => '0',
            // I may be confused with 1 or l
            'I' if rand::random::<bool>() => {
                if rand::random::<bool>() {
                    '1'
                } else {
                    'l'
                }
            }
            // B may be confused with 8
            'B' if rand::random::<bool>() => '8',
            _ => c,
        }
    }
    input.chars().map(alter_char).collect()
}
```

[`data_encoding`]: https://docs.rs/data-encoding/*/data_encoding/
