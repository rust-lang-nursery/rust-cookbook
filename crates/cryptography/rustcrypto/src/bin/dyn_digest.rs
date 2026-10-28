use data_encoding::HEXLOWER;
use sha2::digest::DynDigest;
use sha2::{Sha256, Sha384, Sha512};

/// Looks up a hash function by name, like `EVP_get_digestbyname` in OpenSSL.
fn hasher_by_name(name: &str) -> Option<Box<dyn DynDigest>> {
    match name {
        "sha256" => Some(Box::new(Sha256::default())),
        "sha384" => Some(Box::new(Sha384::default())),
        "sha512" => Some(Box::new(Sha512::default())),
        _ => None,
    }
}

fn main() {
    // The algorithm names could just as well come from a config file.
    for name in ["sha256", "sha384", "sha512", "md5"] {
        match hasher_by_name(name) {
            Some(mut hasher) => {
                hasher.update(b"hello ");
                hasher.update(b"world");
                println!("{name}: {}", HEXLOWER.encode(&hasher.finalize()));
            }
            None => println!("{name}: unsupported algorithm"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_algorithm_at_runtime() {
        let mut hasher = hasher_by_name("sha256").expect("sha256 is supported");
        hasher.update(b"hello world");
        assert_eq!(
            HEXLOWER.encode(&hasher.finalize()),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        assert_eq!(hasher_by_name("sha512").map(|h| h.output_size()), Some(64));
        assert!(hasher_by_name("md5").is_none());
    }
}
