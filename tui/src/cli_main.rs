use tracing::level_filters::LevelFilter;
use tracing_subscriber::{
    fmt,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

mod cli;

fn setup_logging() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    setup_logging()?;
    
    println!("🕸️  Swoop CLI - High-Performance Web Scraper");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    
    cli::run_cli().await?;
    
    Ok(())
}
