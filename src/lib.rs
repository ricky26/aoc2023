use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

pub mod find_numbers;

pub mod ascii_grid;

pub fn bootstrap() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy())
        .init();
}
