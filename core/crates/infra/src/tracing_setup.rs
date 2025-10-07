//! Tracing 初始化

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// 初始化 tracing
pub fn init_tracing(level: &str, format: &str) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level));

    let subscriber = tracing_subscriber::registry().with(env_filter);

    match format {
        "json" => {
            subscriber
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        }
        _ => {
            subscriber
                .with(tracing_subscriber::fmt::layer().pretty())
                .init();
        }
    }
}
