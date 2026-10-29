use tracing::{info, info_span, warn};

fn main() {
    tracing_subscriber::fmt().init();

    let span = info_span!("request", id = 7);
    let _guard = span.enter();
    info!(user = "ferris", "handling request");
    warn!("something looks off");
}
