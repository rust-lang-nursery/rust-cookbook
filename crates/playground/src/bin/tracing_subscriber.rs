use tracing_subscriber::EnvFilter;

fn main() {
    // RUST_LOG overrides the default filter.
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug")),
        )
        .with_target(false)
        .init();

    tracing::debug!("a debug event");
    tracing::info!(answer = 42, "an info event");
}
