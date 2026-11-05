
use axum::{Router, http::StatusCode, response::IntoResponse, routing::get , Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::time::Instant;

mod v1;

use crate::START_TIME;

pub fn init() -> Router {
  let router = Router::new()
    .nest(
      "/api",
      Router::new()
      .route("/health", get(get_health))
      .nest("/v1", v1::routes()),
    )
    .fallback(handler_404);

  return router;
}

#[derive(Serialize)]
pub struct HealthStatus {
  status: &'static str,
  time: DateTime<Utc>,
  uptime: String,
}

pub async fn get_health() -> (StatusCode, Json<HealthStatus>) {
  let secs = START_TIME
    .get_or_init(Instant::now)
    .elapsed()
    .as_secs();

  let (hours , minutes , seconds) = (secs / 3600, (secs % 3600) / 60,  secs % 60);
  let status = HealthStatus {
    status: "ok" ,
    time: Utc::now(),
    uptime: format!("{}h {}m {}s", hours, minutes, seconds),
  };

  return (
    StatusCode::OK, 
    Json(status)
  );
}

async fn handler_404(uri: axum::http::Uri) -> impl IntoResponse {
  return (
    StatusCode::NOT_FOUND, 
    format!("No end point found for path: {}", uri)
  );
}
