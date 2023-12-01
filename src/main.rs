mod commands;

use commands::Commands;
use gooby::prelude::*;

i18n!("locales", fallback = "EN");

#[tokio::main]
async fn main() -> GoobyResult {
  Gooby::subscribe_tracing(tracing::Level::INFO)?;

  Gooby::set_locale(Locale::TR)?;

  let config = Gooby::load_config()?;
  let intents = GoobyIntents::non_privileged() | GoobyIntents::MESSAGE_CONTENT;

  Gooby::new(config)
    .options("?", Commands::all(), None)
    .intents(intents)
    .serve()
    .await?;

  Ok(())
}
