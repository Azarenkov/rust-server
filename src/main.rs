mod domain;
mod adapters;
mod infrastructure;
mod application;

use std::error::Error;
use adapters::messaging::fcm_adapter::FcmAdapter;
use infrastructure::{db, web_server, firebase_messaging};
use tokio::sync::mpsc;
use tokio::task;
use tokio::time::{sleep, Duration};
use application::services::sync_service::SyncService;
use application::repositories::sync_service_abstract::SyncServiceAbstract;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let messaging_client = firebase_messaging::get_messaging_service("service_account_key.json".to_string()).await?;
    let (tx, mut rx) = mpsc::channel::<FcmAdapter>(64);
    
    task::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = messaging_client.send(message).await {
                eprintln!("Error sending message: {:?}", e);
            }
        }
    });
    
    let db = db::get_database().await;

    let service = SyncService::new(db.clone());

    tokio::spawn(async move {
        loop {
            
            if let Err(e) = service.sync_all_data(Some(tx.clone())).await {
                sleep(Duration::from_secs(10)).await;
                println!("{:?}", e);
                continue;
            }
            
            sleep(Duration::from_secs(10)).await;
        }
    });

    web_server::get_web_server(db.clone()).await?;

    Ok(())
}
