use std::time::Duration;
use tokio::{sync::mpsc, task, time::sleep};
use crate::{adapters::{db::db_connection::get_database, messaging::{fcm_adapter::FcmAdapter, fcm_connection::get_messaging_service}}, application::{interfaces::sync_service_abstract::SyncServiceAbstract, services::sync_service::SyncService}};
use std::error::Error;
mod web_server;


pub async fn server() -> Result<(), Box<dyn Error>>{
    let messaging_client = get_messaging_service("service_account_key.json".to_string()).await?;
    let (tx, mut rx) = mpsc::channel::<FcmAdapter>(64);
    
    task::spawn(async move {
        while let Some(message) = rx.recv().await {
            if let Err(e) = messaging_client.send(message.message).await {
                eprintln!("Error sending message: {:?}", e);
            }
        }
    });
    
    let db = get_database().await;

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