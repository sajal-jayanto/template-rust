use sqlx::PgPool;
use axum::http::StatusCode;

pub struct SampleService;

impl SampleService {
  pub async fn get_all(pool: &PgPool) -> Result<(), (StatusCode, String)> {
    // Example query – replace with your actual logic
    // let rows = sqlx::query!("SELECT * FROM your_table")
    //     .fetch_all(pool)
    //     .await
    //     .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // TODO: map rows to your domain model and return them
    Ok(())
  }
}