use tracing::level_filters::LevelFilter;
use tracing_subscriber::{EnvFilter, fmt::time::LocalTime};

pub fn init_logger() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .with_timer(LocalTime::rfc_3339())
        .init();
}
