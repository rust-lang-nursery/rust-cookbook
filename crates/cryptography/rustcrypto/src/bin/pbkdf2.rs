use pbkdf2::Pbkdf2;
use pbkdf2::password_hash::{PasswordHasher, PasswordVerifier};
use pbkdf2::phc::PasswordHash;

fn main() -> Result<(), pbkdf2::password_hash::Error> {
    let password = "Guess Me If You Can!";

    // A random salt is generated and stored in the PHC string along with the
    // algorithm and its parameters.
    let hash = Pbkdf2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    println!("PBKDF2 hash: {hash}");

    let parsed = PasswordHash::new(&hash)?;
    assert!(
        Pbkdf2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    );
    assert!(
        Pbkdf2::default()
            .verify_password(b"Definitely not the correct password", &parsed)
            .is_err()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hashes_and_verifies() -> Result<(), pbkdf2::password_hash::Error> {
        let hash = Pbkdf2::default().hash_password(b"secret")?.to_string();
        assert!(hash.starts_with("$pbkdf2-sha256$"));
        let parsed = PasswordHash::new(&hash)?;
        assert!(
            Pbkdf2::default()
                .verify_password(b"secret", &parsed)
                .is_ok()
        );
        assert!(
            Pbkdf2::default()
                .verify_password(b"wrong", &parsed)
                .is_err()
        );
        Ok(())
    }
}
