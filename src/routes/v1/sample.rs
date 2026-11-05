use axum::{Router, routing::get};
use crate::handlers::sample;

pub fn routes() -> Router {
  Router::new()
    .route(
      "/",
      get( sample::get_all).post(|| async { "Hello, World!" }),
    )
    .route(
      "/{id}",
      get(|| async { "Hello, World!" })
        .put(|| async { "Hello, World!" })
        .delete(|| async { "Hello, World!" }),
    )
}
