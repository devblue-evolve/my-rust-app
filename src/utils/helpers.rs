use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::EnvFilter;

pub fn setup_logging(log_level: &str) -> Result<(), Box<dyn std::error::Error>> {
    let env_filter = EnvFilter::from_default_env()
        .add_directive(log_level.parse()?)
        .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into());

    tracing_subscriber::fmt()
        .with_target(true)
        .with_level(true)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_env_filter(env_filter)
        .init();

    Ok(())
}
