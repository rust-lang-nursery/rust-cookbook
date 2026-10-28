use anyhow::Result;
use data_encoding::HEXLOWER;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{self, Write};

/// A writer that hashes everything written through it.
struct HashingWriter<W: Write, D: Digest> {
    inner: W,
    hasher: D,
}

impl<W: Write, D: Digest> HashingWriter<W, D> {
    fn new(inner: W) -> Self {
        Self {
            inner,
            hasher: D::new(),
        }
    }

    fn finish(self) -> (W, Vec<u8>) {
        (self.inner, self.hasher.finalize().to_vec())
    }
}

impl<W: Write, D: Digest> Write for HashingWriter<W, D> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        // Only hash the bytes the inner writer actually accepted.
        let written = self.inner.write(buf)?;
        self.hasher.update(&buf[..written]);
        Ok(written)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}

fn main() -> Result<()> {
    let dir = tempfile::tempdir()?;
    let path = dir.path().join("copy.txt");

    // Copy a reader into a file and get its digest without reading it back.
    let mut source = &b"We will generate a digest of this text"[..];
    let mut writer = HashingWriter::<_, Sha256>::new(File::create(&path)?);
    io::copy(&mut source, &mut writer)?;
    let (_file, digest) = writer.finish();

    println!(
        "Wrote {} and its SHA-256 digest is {}",
        path.display(),
        HEXLOWER.encode(&digest)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_matches_written_bytes() -> Result<()> {
        let mut writer = HashingWriter::<_, Sha256>::new(Vec::new());
        writer.write_all(b"hello ")?;
        writer.write_all(b"world")?;
        let (written, digest) = writer.finish();
        assert_eq!(written, b"hello world");
        assert_eq!(digest, Sha256::digest(b"hello world").to_vec());
        Ok(())
    }
}
