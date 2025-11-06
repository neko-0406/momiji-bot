use serenity::{Client, all::{EventHandler, GatewayIntents, Ready, Context}};
use std::env;

use crate::memo::memo_handler::MemoHandler;
mod memo;
mod wether;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("DISCORDBOT_TOKEN").expect("トークンの読み込みに失敗しました");

    let intents = GatewayIntents::all();

    let mut client = Client::builder(&token, intents)
        .event_handler(ReadyEvent)
        .event_handler(MemoHandler)
        .await
        .expect("クライアントの初期化に失敗しました");

    if let Err(why) = client.start().await {
        println!("クライアントの起動に失敗しました: {:?}", why);
    }
}

struct ReadyEvent;
#[serenity::async_trait]
impl EventHandler for ReadyEvent {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} としてログインしました!", ready.user.name);
        ctx.online();
    }
}

