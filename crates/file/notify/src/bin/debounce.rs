use notify::RecursiveMode;
use notify_debouncer_full::new_debouncer;
use std::path::PathBuf;
use std::sync::mpsc;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));

    let (tx, rx) = mpsc::channel();
    let mut debouncer = new_debouncer(Duration::from_secs(1), None, tx)?;
    debouncer.watch(&path, RecursiveMode::Recursive)?;

    println!("watching {}. Press Ctrl-C to stop.", path.display());

    for result in rx {
        match result {
            Ok(events) => {
                for event in events {
                    println!("{:?}: {:?}", event.kind, event.paths);
                }
            }
            Err(errors) => {
                for error in errors {
                    eprintln!("watch error: {error:?}");
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn a_burst_collapses_into_one_batch() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let (tx, rx) = mpsc::channel();
        let mut debouncer = new_debouncer(Duration::from_millis(200), None, tx)?;
        debouncer.watch(dir.path(), RecursiveMode::Recursive)?;

        let file = dir.path().join("log.txt");
        for i in 0..5 {
            fs::write(&file, format!("line {i}\n"))?;
        }

        let events = rx
            .recv_timeout(Duration::from_secs(5))?
            .map_err(|errors| format!("watch errors: {errors:?}"))?;
        assert!(events
            .iter()
            .any(|event| event.paths.iter().any(|p| p.ends_with("log.txt"))));
        Ok(())
    }
}
