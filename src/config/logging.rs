use tracing_subscriber::EnvFilter;

pub fn init() {
  tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info")) // default level
    )
    .with_target(true)      // show module path
    .with_thread_ids(true)  // useful with async
    .init();
}