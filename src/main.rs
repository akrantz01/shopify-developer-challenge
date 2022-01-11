use axum::{routing::get, Router, Server};
use eyre::WrapErr;
use std::env;
use tracing::info;

mod logging;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().wrap_err("failed to find .env configuration file")?;
    tracing_subscriber::fmt::init();

    // Get the listen address
    let address = env::var("ADDRESS")
        .wrap_err("missing ADDRESS environment variable")?
        .parse()
        .wrap_err("invalid listen address format")?;

    // Setup the routes
    let router = Router::new()
        .route("/", get(|| async { "hello world" }))
        .layer(logging::layer());

    // Start the server
    info!(%address, "listening and ready to handle requests");
    Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
