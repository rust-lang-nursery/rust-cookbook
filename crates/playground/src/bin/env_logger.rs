fn main() {
    // RUST_LOG overrides this default.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    log::debug!("a debug message");
    log::info!("an info message");
    log::warn!("a warning");
}
