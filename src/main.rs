#![warn(clippy::pedantic)]

use std::net::{Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::{Context, Result};
use axum::Router;
use axum::routing::{get, put};
use clap::Parser;
use log::info;
use tokio::sync::RwLock;

use self::dbase::Database;

mod dbase;
mod route;
mod types;

/// Bookmarks API.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Port to listen for connections.
    #[arg(short, long)]
    #[arg(default_value_t = 3000)]
    port: u16,

    /// Path to serialized bookmark data.
    #[arg(value_name = "BOOKMARKS")]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse args
    let args = Cli::parse();
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load bookmark data
    let db = Database::open(args.path).await?;
    info!("loaded {} bookmarks", db.view().len());

    // Build our application with routes
    let app = Router::new()
        .route("/", get(route::root).put(route::push))
        .route("/sync", put(route::sync))
        .route("/tags", get(route::tags))
        .route("/categories", get(route::categories))
        .with_state(Arc::new(RwLock::new(db)));

    // Run the application on the specified port
    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, args.port));
    info!("listening on {addr}");
    let sock = tokio::net::TcpListener::bind(addr)
        .await
        .context("failed to bind to socket")?;
    axum::serve(sock, app)
        .await
        .context("failed to serve application")?;

    Ok(())
}
