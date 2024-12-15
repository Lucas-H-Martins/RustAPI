use std::env;

use tracing::{info, level_filters::LevelFilter};

use crate::errors::Errors;

pub fn configure_logger() -> Result<(), Errors> {
    // Lê a variável de ambiente LOG_LEVEL. Se não estiver definida, usa "info" como padrão.
    let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());

    // Converte o valor da variável de ambiente para LevelFilter.
    let level_filter = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::TRACE,
        "debug" => LevelFilter::DEBUG,
        "info" => LevelFilter::INFO,
        "warn" => LevelFilter::WARN,
        "error" => LevelFilter::ERROR,
        _ => {
            eprintln!(
                "LOG_LEVEL invalid: '{}', using 'info' as default.",
                log_level
            );
            LevelFilter::DEBUG
        }
    };

    tracing_subscriber::fmt()
        .with_max_level(level_filter)
        .init();

    info!("running with log level {}", log_level);
    Ok(())
}
