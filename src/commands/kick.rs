use super::util;
use gooby::prelude::*;

#[poise::command(
  slash_command,
  prefix_command,
  track_edits,
  required_permissions = "KICK_MEMBERS"
)]
pub async fn kick(
  ctx: PoiseContext<'_>,
  #[description = "Target member"] target: serenity::Member,
  #[description = "Kick reason"] reason: Option<String>,
) -> PoiseResult {
  let result = match reason {
    Some(reason) => target.kick_with_reason(ctx, reason.as_str()).await,
    None => target.kick(ctx).await,
  };

  let reply_content = match result {
    Ok(_) => t!("command.kick.ok", id = target.user.id),
    Err(_) => t!("command.kick.err", id = target.user.id),
  };
  util::reply_temp_ephemeral(ctx, reply_content, 2500).await?;

  Ok(())
}
