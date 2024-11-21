use mongodb::{Client, options::ClientOptions, Database};
use std::env;

pub async fn get_database() -> Database {
    let mongo_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    let client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    client.database("main")
}