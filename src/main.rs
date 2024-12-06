mod domain;
mod adapters;
mod infrastructure;
mod application;

use std::error::Error;
use std::{env, thread};
use actix_web::{App, HttpServer};
use application::services::actix_service::{check_token, get_deadlines, get_grades};
use infrastructure::db::get_database;
use tokio::time::{sleep, Duration};
use application::services::{actix_service::get_user_info, actix_service::get_courses, sync_service::SyncService};
use application::repositories::sync_service_abstract::SyncServiceAbstract;
use actix_web::web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let client = fcm::Client::new();

    let mut notification_builder = fcm::NotificationBuilder::new();
    notification_builder.title("Hey!");
    notification_builder.body("Do you want to catch up later?");
    
    let notification = notification_builder.finalize();
    let fmc = env::var("FMC").expect("You must set the FMC environment var!");
    let mut message_builder = fcm::MessageBuilder::new(&fmc, "enCWYWBmEE1ckS-g2aYNEr:APA91bFbklR52axzKnUZwgs7TdSEPBQFvLxyvbOJ9vTov3SidyE6i69yj2WQhhW899UngHMz18X-7g4rx5pMWsf36ycOuJyQZK1yqiCQRYXwxnUe9sJIWAc");
    message_builder.notification(notification);
    
    let response = client.send(message_builder.finalize()).await?;
    println!("Sent: {:?}", response);

    let db = get_database().await;

    let service = SyncService::new(db.clone());

    tokio::spawn(async move {
        loop {
            
            if let Err(e) = service.sync_all_data().await {
                sleep(Duration::from_secs(10)).await;
                println!("{:?}", e);
                continue;
            }
            
            sleep(Duration::from_secs(10)).await;
        }
    });

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(check_token)
            .service(get_user_info)
            .service(get_courses)
            .service(get_grades)
            .service(get_deadlines)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

// async fn send_notification() {

// }

