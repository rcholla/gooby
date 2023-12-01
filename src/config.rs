use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
  pub gooby_token: String,
  #[serde(default)]
  pub environment: Environment,
}

#[derive(strum::EnumIs, Deserialize, Debug, Clone, PartialEq)]
pub enum Environment {
  Production,
  Development,
}

impl Default for Environment {
  fn default() -> Self {
    Self::Production
  }
}

#[derive(strum::AsRefStr)]
pub enum Locale {
  TR,
  EN,
}
