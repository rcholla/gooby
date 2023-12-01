use crate::prelude::*;
use dyn_fmt::AsStrFormatExt;
use strum::EnumMessage;
use tracing_error::SpanTrace;

pub type GoobyResult<T = (), E = GoobyError> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct GoobyError {
  pub error_type: GoobyErrorType,
  pub inner: anyhow::Error,
  pub context: tracing_error::SpanTrace,
}

impl std::fmt::Display for GoobyError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}: ", self.error_type)?;
    writeln!(f, "{:?}", self.inner)?;
    std::fmt::Display::fmt(&self.context, f)
  }
}

impl<T> From<T> for GoobyError
where
  T: Into<anyhow::Error>,
{
  fn from(err: T) -> Self {
    let inner = err.into();

    Self {
      error_type: GoobyErrorType::Unknown(f!("{inner}")),
      inner,
      context: SpanTrace::capture(),
    }
  }
}

#[rustfmt::skip]
#[derive(Debug, strum::Display, EnumMessage)]
pub enum GoobyErrorType {
  #[strum(message = "Unable to load config")] LoadConfig,
  #[strum(message = "Unable to subscribe tracing")] SubscribeTracing,
  #[strum(message = "Unable to set locale to '{}'")] SetLocale(String),
  #[strum(message = "Unable to start Gooby")] Serve,
  Unknown(String),
}

impl GoobyErrorType {
  fn as_anyhow(&self) -> anyhow::Error {
    match &self {
      Self::Unknown(reason) => anyhow!("{reason}"),
      Self::SetLocale(locale) => anyhow!(self.get_message().unwrap().format(&[locale])),
      _ => anyhow!("{}", self.get_message().unwrap_or("None")),
    }
  }
}

impl From<GoobyErrorType> for GoobyError {
  fn from(error_type: GoobyErrorType) -> Self {
    let inner = error_type.as_anyhow();

    Self {
      error_type,
      inner,
      context: SpanTrace::capture(),
    }
  }
}

pub trait GoobyErrorExt<T, E> {
  fn yeets(self, error_type: GoobyErrorType) -> GoobyResult<T>;
}

impl<T, E> GoobyErrorExt<T, E> for std::result::Result<T, E>
where
  E: Into<anyhow::Error>,
{
  fn yeets(self, error_type: GoobyErrorType) -> GoobyResult<T> {
    let inner = error_type.as_anyhow();

    self.map_err(|_| GoobyError {
      error_type,
      inner,
      context: SpanTrace::capture(),
    })
  }
}

impl<T, E> GoobyErrorExt<T, E> for GoobyResult<T> {
  fn yeets(self, error_type: GoobyErrorType) -> GoobyResult<T> {
    self.map_err(|mut err| {
      err.error_type = error_type;
      err
    })
  }
}

pub trait GoobyErrorExt2<T> {
  fn yeets(self, error_type: GoobyErrorType) -> GoobyResult<T>;
}

impl<T> GoobyErrorExt2<T> for std::option::Option<T> {
  fn yeets(self, error_type: GoobyErrorType) -> GoobyResult<T> {
    self.ok_or(error_type.into())
  }
}
