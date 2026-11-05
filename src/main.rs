use tokio::net::TcpListener;
use tracing::{error, info};

use config::{env, logger};
use std::net::SocketAddr;
use std::sync::OnceLock;
use std::time::Instant;

mod config;
mod db;
mod routes;
mod handlers;

static START_TIME: OnceLock<Instant> = OnceLock::new();

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  logger::init();

  let db = db::get_connection().await;
  let app = routes::init(db);

  let port = env::Var::new("PORT")
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
