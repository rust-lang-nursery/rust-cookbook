#[cfg(target_os = "linux")]
use syslog::{BasicLogger, Facility, Formatter3164};

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "My app name".into(),
        pid: std::process::id(),
    };

    let logger = syslog::unix(formatter)?;
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))?;
    log::set_max_level(log::LevelFilter::Debug);

    log::debug!("this is a debug {}", "message");
    log::error!("this is an error!");
    Ok(())
}

#[cfg(not(target_os = "linux"))]
fn main() {
    println!("So far, only Linux systems are supported.");
}
