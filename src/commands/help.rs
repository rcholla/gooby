use gooby::prelude::*;

#[poise::command(slash_command, prefix_command, track_edits)]
pub async fn help(
  ctx: PoiseContext<'_>,
  #[description = "Specific command to show help about"]
  #[autocomplete = "poise::builtins::autocomplete_command"]
  command: Option<String>,
) -> PoiseResult {
  poise::builtins::help(
    ctx,
    command.as_deref(),
    poise::builtins::HelpConfiguration::default(),
  )
  .await?;

  Ok(())
}
