pub struct EnvGetter {
  key: String,
  default: Option<String>,
}

impl EnvGetter {
  pub fn new(key: impl Into<String>) -> Self {
    return Self { key: key.into(), default: None }
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
      .or_else(|| { self.default.and_then(|d| d.parse().ok())
    });
    return val;
  }
}