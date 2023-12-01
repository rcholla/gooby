use gooby::prelude::*;

pub(super) async fn reply_temp_ephemeral<C>(
  ctx: PoiseContext<'_>,
  content: C,
  ms: u64,
) -> PoiseResult
where
  C: Into<String>,
{
  let reply = ctx
    .send(|r| r.content(content.into()).ephemeral(true))
    .await?;
  tokio::time::sleep(std::time::Duration::from_millis(ms)).await;
  reply.delete(ctx).await?;

  Ok(())
}
