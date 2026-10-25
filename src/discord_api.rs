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
const APP_NAME: &str = "APP_NAME";
const GITHUB_URL: &str = "GITHUB_URL";

#[derive(Default)]
pub struct DiscordAPI {
    pub token: String,
    pub end_point: String,
    pub api_version: String,
    pub app_name: String,
    pub github_url: String,
}

impl DiscordAPI {
    /// #　DiscordAPI構造体の初期化関数
    pub fn init() -> Self {
        dotenv().ok();
        let mut api = Self::default();

        api.token = dotenv::var(DISCORD_BOT_TOKEN).expect("discord api tokenの取得に失敗しました");
        api.end_point = DISCORD_API_ENDPOINT.to_owned();
        api.api_version = DISCORD_API_VERSION.to_owned();

        if let Ok(app_name) = dotenv::var(APP_NAME) {
            api.app_name = app_name;
        }
        if let Ok(github_url) = dotenv::var(GITHUB_URL) {
            api.github_url = github_url;
        }

        println!("discord api endpoint: {}", &api.end_point);
        println!("discord api version: {}", &api.api_version);

        if !api.app_name.is_empty() {
            println!("app name: {}", &api.app_name);
        } else {
            println!("app name: {}", "None");
        }

        if !api.github_url.is_empty() {
            println!("github link: {}", &api.github_url);
        } else {
            println!("github link: {}", "None");
        }

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
    /// User Agentを作成する
    pub fn create_user_agent(&self) -> String {
        format!(
            "DiscordBot ({}, {})",
            &self.github_url,
            env!("CARGO_PKG_VERSION")
        )
    }
    /// ヘッダに付ける認証文字列を返す
    pub fn get_token(&self) -> String {
        format!("Bot {}", &self.token)
    }
}
