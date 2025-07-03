## Sign and verify a message with HMAC digest

[![ring-badge]][ring] [![cat-cryptography-badge]][cat-cryptography]

The [`hmac::sign`] method is used to calculate the HMAC digest (also called a tag) of the message using the provided key.
The resulting [`hmac::Tag`] structure contains the raw bytes of the HMAC,
which can later be verified with[`hmac::verify`] to ensure the message has not been tampered with and comes from a trusted source.

```rust,edition2018
use ring::{hmac, rand};
use ring::rand::SecureRandom;
use ring::error::Unspecified;

fn main() -> Result<(), Unspecified> {
    let mut key_value = [0u8; 48];
    let rng = rand::SystemRandom::new();
    rng.fill(&mut key_value)?;
    let key = hmac::Key::new(hmac::HMAC_SHA256, &key_value);

    let message = "Legitimate and important message.";
    let signature: hmac::Tag = hmac::sign(&key, message.as_bytes());
    hmac::verify(&key, message.as_bytes(), signature.as_ref())?;

    Ok(())
}
```

[`ring::hmac`]: https://docs.rs/ring/*/ring/hmac/index.html

[`hmac::sign`]: https://docs.rs/ring/*/ring/hmac/fn.sign.html

[`hmac::Tag`]: https://docs.rs/ring/*/ring/hmac/struct.Tag.html

[`hmac::verify`]: https://docs.rs/ring/*/ring/hmac/fn.verify.html
