pub mod env {
  pub struct Var {
    key: String,
    default: Option<String>,
  }

  impl Var {
    pub fn new(key: impl Into<String>) -> Self {
      return Self {
        key: key.into(),
        default: None,
      };
    }

    pub fn default(mut self, value: impl Into<String>) -> Self {
      self.default = Some(value.into());
      return self;
    }

    pub fn get(self) -> String {
      let val = match dotenvy::var(&self.key) {
        Ok(v) => v,
        Err(_) => self.default.unwrap_or_default(),
      };
      return val;
    }

    pub fn get_as<T: std::str::FromStr>(self) -> Option<T> {
      let val = dotenvy::var(&self.key)
        .ok()
        .and_then(|v| v.parse().ok())
        .or_else(|| self.default.and_then(|d| d.parse().ok()));
      return val;
    }
  }
}

pub mod logger {
  use tracing_subscriber::EnvFilter;

  pub fn init() {
    let tracing = tracing_subscriber::fmt()
      .with_env_filter(
        EnvFilter::try_from_default_env()
          .unwrap_or_else(|_| EnvFilter::new("info")),
      )
      .with_target(true)
      .with_thread_ids(true)
      .init();
    return tracing;
  }
}
