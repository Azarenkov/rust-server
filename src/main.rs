mod domain;
mod adapters;
mod infrastructure;
mod application;

use std::error::Error;
use infrastructure::db::get_database;
use tokio::time::{sleep, Duration};
use mongodb::bson::{self};
use application::services::sync_service::SyncService;
use application::repositories::sync_service_abstract::SyncServiceAbstract;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = get_database().await;
    let users: mongodb::Collection<bson::Document> = db.collection("users");

    let token = "711abc349948337f8b97cbb01b76adf5";

    let service = SyncService::new(users.clone());
    service.sync_data_with_database().await;
    service.sync_courses_with_database().await;
    service.sync_grades_with_database().await;
    service.sync_deadlines_with_database().await;

    

    // loop {
    //     match api_client.get_user().await {
    //         Ok(user) => println!("{:#?}", user),
    //         Err(e) => println!("{:#?}", e),
    //     }

    //     sleep(Duration::from_secs(10)).await;
    // }
    Ok(())
}

