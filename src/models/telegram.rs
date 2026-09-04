use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TelegramMessage {
    pub chat_id: String,
    pub text: String,
}
#[derive(Debug, Deserialize)]
pub struct TelegramResponse {
    pub ok: bool,
    pub error_code: Option<i32>,
    pub description: Option<String>,
    pub result: Option<TelegramResult>,
}

#[derive(Debug, Deserialize)]
pub struct TelegramResult {
    pub message_id: i64,
    pub from: TelegramUser,
    pub chat: TelegramChat,
    pub date: i64,
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct TelegramUser {
    pub id: i64,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TelegramChat {
    pub id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[serde(rename = "type")]
    pub chat_type: String,
}