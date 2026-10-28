use argon2::Argon2;
use argon2::password_hash::phc::PasswordHash;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};

fn main() -> Result<(), argon2::password_hash::Error> {
    let password = "Guess Me If You Can!";

    // Argon2id with the default parameters and a random salt. The result is a
    // PHC string such as `$argon2id$v=19$m=...,t=...,p=...$salt$hash`.
    let hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    println!("Argon2 hash: {hash}");

    let parsed = PasswordHash::new(&hash)?;
    assert!(
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    );
    assert!(
        Argon2::default()
            .verify_password(b"Definitely not the correct password", &parsed)
            .is_err()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_verifies() -> Result<(), argon2::password_hash::Error> {
        let hash = Argon2::default().hash_password(b"secret")?.to_string();
        assert!(hash.starts_with("$argon2id$v=19$"));
        let parsed = PasswordHash::new(&hash)?;
        assert!(
            Argon2::default()
                .verify_password(b"secret", &parsed)
                .is_ok()
        );
        assert!(
            Argon2::default()
                .verify_password(b"wrong", &parsed)
                .is_err()
        );
        Ok(())
    }
}
