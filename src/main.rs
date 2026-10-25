use crate::discord_api::DiscordAPI;

mod config;
mod discord_api;

#[tokio::main]
async fn main() {
    let discord_api = DiscordAPI::init();
}
