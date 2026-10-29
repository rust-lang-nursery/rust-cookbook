use argon2::Argon2;
use argon2::password_hash::phc::PasswordHash;
use argon2::password_hash::{PasswordHasher, PasswordVerifier};

fn main() -> Result<(), argon2::password_hash::Error> {
    let hash = Argon2::default().hash_password(b"hunter2")?.to_string();
    println!("{hash}");

    let parsed = PasswordHash::new(&hash)?;
    println!(
        "right password: {}",
        Argon2::default()
            .verify_password(b"hunter2", &parsed)
            .is_ok()
    );
    println!(
        "wrong password: {}",
        Argon2::default()
            .verify_password(b"hunter3", &parsed)
            .is_ok()
    );
    Ok(())
}
