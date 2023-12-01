use super::util;
use gooby::prelude::*;

#[poise::command(
  slash_command,
  prefix_command,
  track_edits,
  required_permissions = "BAN_MEMBERS"
)]
pub async fn ban(
  ctx: PoiseContext<'_>,
  #[description = "Target member"] target: serenity::Member,
  #[description = "Dmd"]
  #[min = 0]
  #[max = 7]
  dmd: u8,
  #[description = "Ban reason"] reason: Option<String>,
) -> PoiseResult {
  let result = match reason {
    Some(reason) => target.ban_with_reason(ctx, dmd, reason.as_str()).await,
    None => target.ban(ctx, dmd).await,
  };

  let reply_content = match result {
    Ok(_) => t!("command.ban.ok", id = target.user.id),
    Err(_) => t!("command.ban.err", id = target.user.id),
  };
  util::reply_temp_ephemeral(ctx, reply_content, 2500).await?;

  Ok(())
}
