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
pub async fn fetch_from_db() -> (String,String){
    let empty_str = "".to_string();
    let mongo_db = MongoDb::get_mongodb();
    let collection = mongo_db.database.collection::<mongodb::bson::Document>("mst_telegram_data");
    let filter = doc! {
        "is_active": true
    };
    if let Ok(Some(user)) = collection.find_one(filter).await {
        let token = user.get_str("telegram_bot_token").unwrap_or("").to_string();
        let chat_id = user.get_str("telegram_bot_chat_id").unwrap_or("").to_string();

        return (token,chat_id);
    }
    return (empty_str.to_string(),empty_str.to_string());
}