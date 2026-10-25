use reqwest::{
    Client,
    header::{AUTHORIZATION, USER_AGENT},
};

use crate::{discord_api::DiscordAPI, discord_api_object::GatewayObject};

mod config;
mod discord_api;
mod discord_api_object;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let discord_api = DiscordAPI::init();

    let client = Client::new();
    let response_result = client
        .get(discord_api.get_bot_gateway_url())
        .header(USER_AGENT, discord_api.create_user_agent())
        .header(AUTHORIZATION, discord_api.get_token())
        .send()
        .await;

    Ok(())
}
