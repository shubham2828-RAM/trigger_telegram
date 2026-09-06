use mongodb::{
    Client,
    Database,
};
#[derive(Clone,Debug)]
pub struct MongoDb {
    pub client: Client,
    pub database: Database,
}