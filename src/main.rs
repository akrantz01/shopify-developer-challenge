use axum::{routing::get, AddExtensionLayer, Router, Server};
use eyre::WrapErr;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool};
use std::{env, str::FromStr};
use tracing::{info, log::LevelFilter, warn};

mod inventory;
mod logging;
mod shipments;
mod util;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    if dotenv::dotenv().is_err() {
        warn!(".env file not found");
    }
    tracing_subscriber::fmt::init();

    // Get the listen address
    let address = env::var("ADDRESS")
        .wrap_err("missing ADDRESS environment variable")?
        .parse()
        .wrap_err("invalid listen address format")?;

    // Connect to the database
    let database_url =
        env::var("DATABASE_URL").wrap_err("missing DATABASE_URL environment variable")?;
    let mut options =
        PgConnectOptions::from_str(&database_url).wrap_err("invalid connection URL format")?;
    options.log_statements(LevelFilter::Debug);
    let pool = PgPool::connect_with(options)
        .await
        .wrap_err("failed connect to Postgres database")?;
    info!("connected to the database");

    // Run database migrations
    sqlx::migrate!()
        .run(&pool)
        .await
        .wrap_err("failed to run database migrations")?;

    // Setup the routes
    let router = Router::new()
        .route("/", get(|| async { "hello world" }))
        .nest("/inventory", inventory::router())
        .nest("/shipments", shipments::router())
        .layer(logging::layer())
        .layer(AddExtensionLayer::new(pool));

    // Start the server
    info!(%address, "listening and ready to handle requests");
    Server::bind(&address)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}
