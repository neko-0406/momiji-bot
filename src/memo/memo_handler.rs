use serenity::all::{EventHandler, Context, Message};

pub struct MemoHandler;

#[serenity::async_trait]
impl EventHandler for MemoHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with("!memo") { return }
        let content = "メモしました。";
        msg.reply(ctx, content).await.unwrap();
    }
}
