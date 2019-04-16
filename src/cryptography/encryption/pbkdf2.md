<a name="ex-pbkdf2"></a>
## Salt and hash a password with PBKDF2

[![ring-badge]][ring] [![data-encoding-badge]][data-encoding] [![cat-cryptography-badge]][cat-cryptography]

Uses [`ring::pbkdf2`] to hash a salted password using the PBKDF2 key derivation
function [`pbkdf2::derive`].  Verifies the hash is correct with
[`pbkdf2::verify`]. The salt is generated using
[`SecureRandom::fill`], which fills the salt byte array with
securely generated random numbers.

```rust
extern crate ring;
extern crate data_encoding;

use data_encoding::HEXUPPER;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};

fn main() -> Result<(), Unspecified> {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    const N_ITER: u32 = 100_000;
    let rng = rand::SystemRandom::new();

    let mut salt = [0u8; CREDENTIAL_LEN];
    rng.fill(&mut salt)?;

    let password = "Guess Me If You Can!";
    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        &digest::SHA512,
        N_ITER,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    println!("Salt: {}", HEXUPPER.encode(&salt));
    println!("PBKDF2 hash: {}", HEXUPPER.encode(&pbkdf2_hash));

    let should_succeed = pbkdf2::verify(
        &digest::SHA512,
		N_ITER,
        &salt,
        password.as_bytes(),
        &pbkdf2_hash,
    );
    let wrong_password = "Definitely not the correct password";
    let should_fail = pbkdf2::verify(
        &digest::SHA512,
        N_ITER,
        &salt,
        wrong_password.as_bytes(),
        &pbkdf2_hash,
    );

    assert!(should_succeed.is_ok());
    assert!(!should_fail.is_ok());

    Ok(())
}
```

[`pbkdf2::derive`]: https://briansmith.org/rustdoc/ring/pbkdf2/fn.derive.html
[`pbkdf2::verify`]: https://briansmith.org/rustdoc/ring/pbkdf2/fn.verify.html
[`ring::pbkdf2`]: https://briansmith.org/rustdoc/ring/pbkdf2/index.html
[`SecureRandom::fill`]: https://briansmith.org/rustdoc/ring/rand/trait.SecureRandom.html#tymethod.fill
