pub mod services;
pub mod models;
pub mod utility;

use std::env;
use services::telegram::send_telegram_message;
use crate::utility::ist_date_time_format::get_ist_date_time;
use crate::models::mongodb::MongoDb;
use crate::services::mongodb_config;
use crate::services::mongodb_query_fun::insert_into_db;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();


    match MongoDb::init_mongodb().await {

        Ok(_) => {
            println!("MongoDB initialization successful");
        }

        Err(error) => {
            eprintln!(
                "Failed to initialize MongoDB Atlas: {}",
                error
            );

            return;
        }
    }

    insert_into_db().await;


    let chat_id = env::var("TELEGRAM_BOT_CHAT_ID").unwrap_or("".to_string()).to_string();
    let name= "Shubham Sharma";
    let mobile = Some("7302720085".to_string());
    let query = "I need to reach out you please help me out";
    let date_time = get_ist_date_time();
    let project_link = "https://shubham2828-ram.github.io/push_notification_doc/";
    let message = format!(
        "🔴 User Notification Alert\n\n\
            Name: {}\n\
            Mob: {}\n\
            Query: {}\n\
            Date Time: {}\n\n\
            Project Link: {}",
        name,
        mobile.unwrap_or_else(|| "No".to_string()),
        query,
        date_time,
        project_link
    );
    match send_telegram_message( message.as_str()).await
    {
        Ok(_) => {
            println!("Telegram message sent successfully");
        }

        Err(error) => {
            println!("Failed to send Telegram message: {}", error);
        }
    }
}