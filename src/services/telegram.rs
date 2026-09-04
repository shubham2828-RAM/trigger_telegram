use reqwest::Client;
use std::env;
use crate::models::telegram::{TelegramMessage, TelegramResponse};

pub async fn send_telegram_message(chat_id: &str, message: &str, ) -> Result<(), Box<dyn std::error::Error>> {

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap_or("".to_string()).to_string();
    println!("token -> {:?}",token.to_string());

    let url = format!("https://api.telegram.org/bot{}/sendMessage", token);

    let payload = TelegramMessage {
        chat_id: chat_id.to_string(),
        text: message.to_string(),
    };

    let client = Client::new();
    let response = client.post(&url).json(&payload).send().await?;
    let telegram_response: TelegramResponse = response.json().await?;
    println!("Telegram response: {:?}", telegram_response);
    if telegram_response.ok{
        return Ok(());
    }else {
        Err("Telegram API returned an error!".into())
    }
}