mod app;

use crate::app::build_app;
use anyhow::Result;
use std::env::var;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::Level;

#[::tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv()?;
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    let app = build_app(
        "web-server/public",
        var("DATABASE_URL")?,
        (var("ADMIN_USER")?, var("ADMIN_PASSWORD")?),
    )
    .await?
    .layer(CompressionLayer::new())
    .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await?;
    tracing::info!("Listening to new connections");
    axum::serve(listener, app).await?;
    Ok(())
}
