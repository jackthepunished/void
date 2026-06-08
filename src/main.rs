use anyhow::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("Starting VOID...");

    let mut app = void::app::App::new()?;
    app.run().await?;

    tracing::info!("VOID exited cleanly.");
    Ok(())
}
