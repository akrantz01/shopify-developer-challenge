use eyre::WrapErr;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().wrap_err("failed to find .env configuration file")?;
    tracing_subscriber::fmt::init();

    // Get the listen address
    let address = env::var("ADDRESS").wrap_err("missing ADDRESS environment variable")?;

    // Start the server
    info!(%address, "listening and ready to handle requests");
    // TODO: actually start it

    Ok(())
}
