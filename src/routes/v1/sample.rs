use axum::{Router, routing::{delete, get, post, put}};

pub fn routes() -> Router {
  let routers = Router::new()
    .route("/", get(|| async { "Hello, World!" }))
    .route("/", post(|| async { "Hello, World!" }))
    .route("/{id}", get(|| async { "Hello, World!" }))
    .route("/{id}", put(|| async { "Hello, World!" }))
    .route("/{id}", delete(|| async { "Hello, World!" }));
  
  return routers;
}