use std::error::Error;
use std::path::PathBuf;
use tempfile::tempdir;
use tokio::fs;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let dir = tempdir()?;
    let paths: Vec<PathBuf> = (0..5)
        .map(|i| dir.path().join(format!("part-{i}.log")))
        .collect();

    for (i, path) in paths.iter().enumerate() {
        fs::write(path, "x".repeat((i + 1) * 1024)).await?;
    }

    let mut tasks: JoinSet<std::io::Result<(PathBuf, usize)>> = JoinSet::new();
    for path in paths {
        tasks.spawn(async move {
            let bytes = fs::read(&path).await?;
            Ok((path, bytes.len()))
        });
    }

    let mut total = 0;
    while let Some(joined) = tasks.join_next().await {
        let (path, len) = joined??;
        println!("{}: {len} bytes", path.display());
        total += len;
    }
    println!("total: {total} bytes");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn reads_every_file() -> Result<(), Box<dyn Error>> {
        let dir = tempdir()?;
        let paths: Vec<PathBuf> = (0..3)
            .map(|i| dir.path().join(format!("f-{i}")))
            .collect();
        for (i, path) in paths.iter().enumerate() {
            fs::write(path, vec![0u8; (i + 1) * 10]).await?;
        }

        let mut tasks: JoinSet<std::io::Result<usize>> = JoinSet::new();
        for path in paths {
            tasks.spawn(async move { Ok(fs::read(&path).await?.len()) });
        }

        let mut total = 0;
        while let Some(joined) = tasks.join_next().await {
            total += joined??;
        }
        assert_eq!(total, 10 + 20 + 30);
        Ok(())
    }
}
