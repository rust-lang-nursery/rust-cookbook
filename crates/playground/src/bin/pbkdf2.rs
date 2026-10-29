use pbkdf2::Pbkdf2;
use pbkdf2::password_hash::{PasswordHasher, PasswordVerifier};
use pbkdf2::phc::PasswordHash;

fn main() -> Result<(), pbkdf2::password_hash::Error> {
    let hash = Pbkdf2::default().hash_password(b"hunter2")?.to_string();
    println!("{hash}");

    let parsed = PasswordHash::new(&hash)?;
    println!(
        "right password: {}",
        Pbkdf2::default()
            .verify_password(b"hunter2", &parsed)
            .is_ok()
    );
    println!(
        "wrong password: {}",
        Pbkdf2::default()
            .verify_password(b"hunter3", &parsed)
            .is_ok()
    );
    Ok(())
}
