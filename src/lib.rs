#![feature(yeet_expr)]

mod config;
mod error;
pub mod prelude;
mod types;

use crate::prelude::*;

i18n!("locales", fallback = "EN");

pub struct Gooby {
  config: Config,
  options: PoiseOptions,
  intents: GoobyIntents,
}

impl Gooby {
  pub fn new(config: Config) -> Self {
    Self {
      config,
      options: PoiseOptions::default(),
      intents: GoobyIntents::default(),
    }
  }

  pub fn load_config() -> GoobyResult<Config> {
    tracing::info!("Loading config");
    dotenvy::dotenv().yeets(GoobyErrorType::LoadConfig)?;
    let config = envy::from_env::<Config>().yeets(GoobyErrorType::LoadConfig)?;

    Ok(config)
  }

  pub fn subscribe_tracing(level: tracing::Level) -> GoobyResult {
    tracing::subscriber::set_global_default(
      tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(level)
        .finish(),
    )
    .yeets(GoobyErrorType::SubscribeTracing)?;

    Ok(())
  }

  pub fn set_locale(locale: Locale) -> GoobyResult {
    let locale = locale.as_ref();

    if !available_locales!().contains(&locale) {
      do yeet GoobyErrorType::SetLocale(locale.into());
    }

    tracing::info!("Setting locale to '{locale}'");
    rust_i18n::set_locale(locale);

    Ok(())
  }

  pub fn options<P>(
    mut self,
    prefix: P,
    commands: Vec<PoiseCommand>,
    opts: Option<PoiseOptions>,
  ) -> Self
  where
    P: Into<String>,
  {
    let opts = opts.unwrap_or_default();

    self.options = poise::FrameworkOptions {
      commands,
      prefix_options: poise::PrefixFrameworkOptions {
        prefix: Some(prefix.into()),
        edit_tracker: Some(poise::EditTracker::for_timespan(
          std::time::Duration::from_secs(3600),
        )),
        ..opts.prefix_options
      },
      on_error: |error| Box::pin(Self::on_error(error)),
      skip_checks_for_owners: self.config.environment.is_development(),
      ..opts
    };

    self
  }

  pub fn intents(mut self, intents: GoobyIntents) -> Self {
    self.intents = intents;
    self
  }

  pub async fn serve(self) -> GoobyResult {
    let Self {
      config,
      options,
      intents,
    } = self;

    tracing::info!("Gooby is getting ready to touch some grass!");
    poise::Framework::builder()
      .token(&config.gooby_token)
      .setup(move |ctx, ready, framework| {
        Box::pin(async move {
          tracing::info!("{} is ready to goob out.", ready.user.name);
          poise::builtins::register_globally(ctx, &framework.options().commands).await?;

          Ok(PoiseData { config })
        })
      })
      .options(options)
      .intents(intents)
      .run()
      .await
      .yeets(GoobyErrorType::Serve)?;

    Ok(())
  }

  async fn on_error(error: PoiseFrameworkError<'_>) {
    match error {
      PoiseFrameworkError::Setup { error, .. } => {
        tracing::error!("Failed to start bot: {:?}", error);
        std::process::exit(1);
      }
      PoiseFrameworkError::Command { error, ctx } => {
        tracing::error!("Error in command `{}`: {:?}", ctx.command().name, error);
      }
      error => {
        if let Err(e) = poise::builtins::on_error(error).await {
          tracing::error!("Error while handling error: {}", e);
        };
      }
    }
  }
}
