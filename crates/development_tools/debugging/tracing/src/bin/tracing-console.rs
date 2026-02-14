use tracing::{debug, error, info, trace, warn};

fn main() {
    tracing_subscriber::fmt::init();

    error!("This is an error!");
    warn!("This is a warning.");
    info!("This is an informational message.");

    // with the default configuration, debug! and trace! messages are not shown
    debug!("This is a debug message.");
    trace!("This is a trace message.");
}

#[test]
fn test_main() {
    main();
}
