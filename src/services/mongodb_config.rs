use mongodb::{
    Client,
    Database,
};
use tokio::sync::OnceCell;
use crate::models::mongodb::MongoDb;
pub static MONGODB: OnceCell<MongoDb> = OnceCell::const_new();


use std::env;

impl MongoDb {

    pub async fn connect() -> Result<Self, mongodb::error::Error> {

        let mongodb_uri = env::var("MONGODB_URI")
            .expect("MONGODB_URI is not set");

        let database_name = env::var("MONGODB_DATABASE")
            .expect("MONGODB_DATABASE is not set");

        println!("Connecting to MongoDB Atlas...");

        let client = Client::with_uri_str(&mongodb_uri).await?;

        let database = client.database(&database_name);

        // Test MongoDB connection
        database.run_command(mongodb::bson::doc! {"ping": 1}).await?;
        println!("MongoDB Atlas connected successfully. Database: {}", database_name);

        Ok(Self {
            client,
            database,
        })
    }
    pub async fn init_mongodb() -> Result<(), mongodb::error::Error> {

        MONGODB
            .get_or_try_init(|| async {
                MongoDb::connect().await
            })
            .await?;

        Ok(())
    }

    pub fn get_mongodb() -> &'static MongoDb {

        MONGODB
            .get()
            .expect("MongoDB has not been initialized")
    }
}