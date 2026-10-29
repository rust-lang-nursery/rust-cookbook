use anyhow::Result;
use argon2::password_hash::phc::PasswordHash;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};
use argon2::{Algorithm, Argon2, Params, Version};

/// The cost parameters new hashes should use.
fn current_argon2() -> Result<Argon2<'static>> {
    let params = Params::new(32 * 1024, 3, 1, None)?;
    Ok(Argon2::new(Algorithm::Argon2id, Version::V0x13, params))
}

/// Checks a login attempt. If the password is right but the stored hash was
/// made with weaker parameters, returns a replacement hash to store.
fn verify_and_upgrade(password: &str, stored: &str) -> Result<Option<String>> {
    let parsed = PasswordHash::new(stored)?;
    // The parameters in the stored hash are used to verify it.
    Argon2::default().verify_password(password.as_bytes(), &parsed)?;

    let current = current_argon2()?;
    let used = Params::try_from(&parsed)?;
    if used.m_cost() < 32 * 1024 || used.t_cost() < 3 {
        let upgraded = current.hash_password(password.as_bytes())?.to_string();
        return Ok(Some(upgraded));
    }
    Ok(None)
}

fn main() -> Result<()> {
    let password = "Guess Me If You Can!";

    // A hash from before the parameters were raised.
    let weak = Params::new(8 * 1024, 1, 1, None)?;
    let old_hash = Argon2::new(Algorithm::Argon2id, Version::V0x13, weak)
        .hash_password(password.as_bytes())?
        .to_string();
    println!("Old hash: {old_hash}");

    match verify_and_upgrade(password, &old_hash)? {
        Some(new_hash) => println!("Upgraded: {new_hash}"),
        None => println!("Hash already up to date"),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrades_only_weak_hashes() -> Result<()> {
        let weak = Params::new(8 * 1024, 1, 1, None)?;
        let old = Argon2::new(Algorithm::Argon2id, Version::V0x13, weak)
            .hash_password(b"secret")?
            .to_string();

        let upgraded = verify_and_upgrade("secret", &old)?.expect("weak hash is upgraded");
        assert!(upgraded.contains("m=32768,t=3,p=1"));
        assert!(verify_and_upgrade("secret", &upgraded)?.is_none());
        assert!(verify_and_upgrade("wrong", &old).is_err());
        Ok(())
    }
}
