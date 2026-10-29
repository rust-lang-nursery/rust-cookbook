use log::{Level, LevelFilter, Log, Metadata, Record};

struct StdoutLogger;

impl Log for StdoutLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

static LOGGER: StdoutLogger = StdoutLogger;

fn main() -> Result<(), log::SetLoggerError> {
    log::set_logger(&LOGGER)?;
    log::set_max_level(LevelFilter::Info);

    log::info!("shown");
    log::debug!("filtered out");
    Ok(())
}
