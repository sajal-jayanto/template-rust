use axum::Router;

mod sample;

pub fn routes() -> Router {
  let routers = Router::new().nest("/sample", sample::routes());

  return routers;
}
