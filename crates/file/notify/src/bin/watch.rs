use notify::event::{EventKind, ModifyKind};
use notify::{RecursiveMode, Watcher};
use std::path::PathBuf;
use std::sync::mpsc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let (tx, rx) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(tx)?;
    watcher.watch(&path, RecursiveMode::Recursive)?;

    println!("watching {}. Press Ctrl-C to stop.", path.display());

    for event in rx {
        let event = event?;
        match event.kind {
            EventKind::Create(_) => println!("created:  {:?}", event.paths),
            EventKind::Modify(ModifyKind::Data(_)) => {
                println!("modified: {:?}", event.paths)
            }
            EventKind::Remove(_) => println!("removed:  {:?}", event.paths),
            _ => {}
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::time::{Duration, Instant};

    #[test]
    fn create_emits_a_create_event() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(tx)?;
        watcher.watch(dir.path(), RecursiveMode::Recursive)?;

        let file = dir.path().join("created.txt");
        fs::write(&file, b"data")?;

        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let timeout = deadline.saturating_duration_since(Instant::now());
            let event = rx.recv_timeout(timeout)??;
            if matches!(event.kind, EventKind::Create(_))
                && event.paths.iter().any(|p| p.ends_with("created.txt"))
            {
                return Ok(());
            }
        }
    }
}
