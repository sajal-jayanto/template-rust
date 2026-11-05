use tokio::net::TcpListener;
use tracing::{info, error};

use crate::config::{env::EnvGetter, logging};
use std::net::SocketAddr;
use std::time::Instant;

mod config;
mod routes;

static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  logging::init();

  let app= routes::init();
  let port = EnvGetter::new("PORT")
    .default("8080")
    .get_as::<u16>()
    .unwrap_or(8080);

  let addr = SocketAddr::from(([127, 0, 0, 1], port));
  let listener = match TcpListener::bind(&addr).await {
    Ok(listener) => listener,
    Err(e) => {
      error!(%addr, error = %e, "Failed to bind");
      std::process::exit(1);
    }
  };
  info!("🚀 Server successfully started on http://{addr}"); 
  if let Err(e) = axum::serve(listener, app).await {
    error!("Server error: {}", e);
    std::process::exit(1);
  }
}
