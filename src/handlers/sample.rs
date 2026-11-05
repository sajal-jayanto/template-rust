use axum::{
  Extension, Json, http::StatusCode,
};
use sqlx::PgPool;

pub async fn get_all(Extension(db): Extension<PgPool>) -> 
  Result<(StatusCode, Json<()>), (StatusCode, Json<()>)> 
{
  // pool;
  todo!()
}