use dotenv::dotenv;
// ========DISCORD API========
// apiのエンドポイント
const DISCORD_API_ENDPOINT: &str = "https://discord.com/api/";
// apiバージョン
const DISCORD_API_VERSION: &str = "v10";
// bot gateway取得
const DISCORD_GET_BOT_GATEWAY: &str = "/gateway/bot";

// ========.env========
const DISCORD_BOT_TOKEN: &str = "DISCORD_BOT_TOKEN";

#[derive(Default)]
pub struct DiscordAPI {
    pub token: String,
    pub end_point: String,
    pub api_version: String,
}

impl DiscordAPI {
    /// #　DiscordAPI構造体の初期化関数
    pub fn init() -> Self {
        dotenv().ok();
        let mut api = Self::default();

        api.token = dotenv::var(DISCORD_BOT_TOKEN).expect("discord api tokenの取得に失敗しました");
        api.end_point = DISCORD_API_ENDPOINT.to_owned();
        api.api_version = DISCORD_API_VERSION.to_owned();

        println!("discord api endpoint: {}", &api.end_point);
        println!("discord api version: {}", &api.api_version);

        api
    }

    /// APIベースURL + APIバージョンのURLを返す
    pub fn get_base_url(&self) -> String {
        format!("{}{}", self.end_point, self.api_version)
    }
    /// Bot Gataway入手用のURLを返す
    pub fn get_bot_gateway_url(&self) -> String {
        format!("{}{}", self.get_base_url(), DISCORD_GET_BOT_GATEWAY)
    }
}
