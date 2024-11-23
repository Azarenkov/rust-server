mod domain;
mod adapters;
mod infrastructure;
mod application;

use std::error::Error;
use tokio::time::{sleep, Duration};
use mongodb::bson::{self};
use infrastructure::{db::get_database, repositories::DbRepositoryAbstract};
use application::services::{sync_data_with_database, sync_courses_with_database};
use adapters::db::db_adapter::DbAdapter;
use adapters::api::client::ApiClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = get_database().await;
    let users: mongodb::Collection<bson::Document> = db.collection("users");

    let token = "711abc349948337f8b97cbb01b76adf5";

    
    sync_data_with_database(users.clone()).await;

    sync_courses_with_database(users.clone()).await;    

    // loop {
    //     match api_client.get_user().await {
    //         Ok(user) => println!("{:#?}", user),
    //         Err(e) => println!("{:#?}", e),
    //     }

    //     sleep(Duration::from_secs(10)).await;
    // }
    Ok(())
}

