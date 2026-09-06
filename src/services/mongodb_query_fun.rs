use mongodb::bson::doc;
use crate::models::mongodb::MongoDb;
use crate::services::mongodb_config;
pub async fn insert_into_db() {

    let mongo_db = MongoDb::get_mongodb();
    let collection = mongo_db.database.collection::<mongodb::bson::Document>("mst_telegram_data");

    let document = doc! {
        "telegram_bot_token": "8899130292:AAG_soeNfSYBHDGJeOaIqdIXaH9cFO9oZf8",
        "telegram_bot_chat_id": "1088744165",
        "is_active": true
    };

    match collection.insert_one(document).await {

        Ok(response) => {
            println!(
                "Document inserted successfully: {:?}",
                response.inserted_id
            );
        }

        Err(error) => {
            println!(
                "Failed to insert document: {}",
                error
            );
        }
    }
}