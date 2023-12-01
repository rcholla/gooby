use super::util;
use gooby::prelude::*;

#[poise::command(
  slash_command,
  prefix_command,
  track_edits,
  required_permissions = "MANAGE_MESSAGES"
)]
pub async fn clear(
  ctx: PoiseContext<'_>,
  #[description = "Amount of message"] amount: u8,
  #[description = "Target member"] target: Option<serenity::Member>,
) -> PoiseResult {
  let from = target.as_ref().map_or_else(String::new, |member| {
    t!("command.clear.from", id = member.user.id)
  });

  let messages = ctx.channel_id().messages(ctx, |messages| messages).await?;
  let deletion_list: Vec<_> = messages
    .into_iter()
    .filter(|message| {
      target
        .as_ref()
        .map_or(true, |member| message.author.id == member.user.id)
    })
    .take(amount.into())
    .collect();

  if deletion_list.is_empty() {
    let reply_content = t!("command.clear.empty_deletion_list", sent_by = from);
    util::reply_temp_ephemeral(ctx, reply_content, 1500).await?;

    return Ok(());
  };

  let result = ctx.channel_id().delete_messages(ctx, &deletion_list).await;

  let reply_content = match result {
    Ok(_) => t!("command.clear.ok", count = deletion_list.len()),
    Err(err) => t!("command.clear.err", sent_by = from, err = err.to_string()),
  };
  util::reply_temp_ephemeral(ctx, reply_content, 5000).await?;

  Ok(())
}
