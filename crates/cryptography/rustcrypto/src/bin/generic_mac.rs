use hmac::{Hmac, KeyInit, Mac};
use sha2::{Sha256, Sha512};

fn sign<M: Mac + KeyInit>(
    key: &[u8],
    message: &[u8],
) -> Result<Vec<u8>, hmac::digest::InvalidLength> {
    let mut mac = <M as KeyInit>::new_from_slice(key)?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Verifies a tag in constant time with whichever MAC `M` names.
fn verify<M: Mac + KeyInit>(key: &[u8], message: &[u8], tag: &[u8]) -> bool {
    match <M as KeyInit>::new_from_slice(key) {
        Ok(mut mac) => {
            mac.update(message);
            mac.verify_slice(tag).is_ok()
        }
        Err(_) => false,
    }
}

fn main() -> Result<(), hmac::digest::InvalidLength> {
    let key = b"my secret and secure key";
    let message = b"input message";

    let tag256 = sign::<Hmac<Sha256>>(key, message)?;
    let tag512 = sign::<Hmac<Sha512>>(key, message)?;
    println!("HMAC-SHA256 tag is {} bytes", tag256.len());
    println!("HMAC-SHA512 tag is {} bytes", tag512.len());

    assert!(verify::<Hmac<Sha256>>(key, message, &tag256));
    // A tag only verifies with the algorithm that made it.
    assert!(!verify::<Hmac<Sha512>>(key, message, &tag256));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_are_algorithm_specific() -> Result<(), hmac::digest::InvalidLength> {
        let tag256 = sign::<Hmac<Sha256>>(b"Jefe", b"what do ya want for nothing?")?;
        assert_eq!(tag256.len(), 32);
        assert!(verify::<Hmac<Sha256>>(
            b"Jefe",
            b"what do ya want for nothing?",
            &tag256
        ));
        assert!(!verify::<Hmac<Sha512>>(
            b"Jefe",
            b"what do ya want for nothing?",
            &tag256
        ));
        assert!(!verify::<Hmac<Sha256>>(b"Jefe", b"something else", &tag256));
        Ok(())
    }
}
