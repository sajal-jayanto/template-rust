use sqlx::{PgPool, postgres::{PgConnectOptions, PgPoolOptions}};
use std::{error::Error, time::Duration};

use tracing::{error, info};
use crate::config::env::EnvGetter;

async fn init() -> Result<PgPool, Box<dyn Error>> {
  let db_hostname = EnvGetter::new("DB_HOST_NAME")
    .default("localhost")
    .get();

  let db_port= EnvGetter::new("DB_PORT")
    .default("5432")
    .get_as::<u16>()
    .unwrap_or(5432);
    
  let db_username =  EnvGetter::new("DB_USERNAME")
    .default("root")
    .get();

  let db_password =  EnvGetter::new("DB_PASSWORD")
    .default("root")
    .get();

  let db_name = EnvGetter::new("DB_NAME")
    .default("test")
    .get();

  let connection_options = PgConnectOptions::new()
    .host(&db_hostname)
    .port(db_port)
    .username(&db_username)
    .password(&db_password)
    .database(&db_name);

  let pool = PgPoolOptions::new()
    .max_connections(20)
    .acquire_timeout(Duration::from_secs(2))
    .connect_with(connection_options)
    .await?;

  Ok(pool)
}

pub async fn get_connection() -> PgPool {
  let pool = match init().await {
    Ok(pool) => pool,
    Err(err) => {
      error!("❌ failed to connect to the database: {err}");
      std::process::exit(1);
    }
  };
  info!("✅ Database connect successfully.");
  return pool;
}
