pub type PoiseResult<T = ()> = std::result::Result<T, PoiseError>;
pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;

pub type PoiseContext<'a> = poise::Context<'a, PoiseData, PoiseError>;

pub type PoiseOptions = poise::FrameworkOptions<PoiseData, PoiseError>;

pub type PoiseFrameworkError<'a> = poise::FrameworkError<'a, PoiseData, PoiseError>;

pub type PoiseCommand = poise::Command<PoiseData, PoiseError>;

pub struct PoiseData {
  pub config: crate::config::Config,
}
