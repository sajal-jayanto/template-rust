use axum::{
  Extension, Json, http::StatusCode,
};
use crate::services::sample::SampleService;
use sqlx::PgPool;


pub async fn get_all(Extension(pool): Extension<PgPool>) -> 
  Result<(StatusCode, Json<()>), (StatusCode, Json<()>)> 
{
  match SampleService::get_all(&pool).await {
    Ok(_) => Ok((StatusCode::OK, Json(()))),
    Err((status, msg)) => Err((status, Json(()))),
  }
}