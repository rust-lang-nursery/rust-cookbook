use syslog::{Facility, Formatter3164};

fn main() {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "playground".into(),
        pid: std::process::id(),
    };

    // The playground has no syslog daemon, so connecting is expected to fail there.
    match syslog::unix(formatter) {
        Ok(mut logger) => {
            if let Err(err) = logger.info("hello from the cookbook") {
                println!("could not write: {err}");
            }
        }
        Err(err) => println!("no syslog available: {err}"),
    }
}
