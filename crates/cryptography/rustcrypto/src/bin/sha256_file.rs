use anyhow::Result;
use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::{BufReader, Read, Write};

/// Hashes everything `reader` yields with any hash that implements [`Digest`].
fn digest_reader<D: Digest, R: Read>(mut reader: R) -> Result<Vec<u8>> {
    let mut hasher = D::new();
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok(hasher.finalize().to_vec())
}

fn main() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("file.txt");

    let mut output = File::create(&path)?;
    write!(output, "We will generate a digest of this text")?;

    let sha256 = digest_reader::<Sha256, _>(BufReader::new(File::open(&path)?))?;
    println!("SHA-256 digest is {}", HEXLOWER.encode(&sha256));

    // Switching algorithms only changes the type parameter.
    let sha512 = digest_reader::<Sha512, _>(BufReader::new(File::open(&path)?))?;
    println!("SHA-512 digest is {}", HEXLOWER.encode(&sha512));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_known_digests() -> Result<()> {
        let sha256 = digest_reader::<Sha256, _>(&b"hello world"[..])?;
        assert_eq!(
            HEXLOWER.encode(&sha256),
            "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"
        );
        let sha512 = digest_reader::<Sha512, _>(&b"hello world"[..])?;
        assert!(HEXLOWER.encode(&sha512).starts_with("309ecc489c12d6eb"));
        Ok(())
    }
}
