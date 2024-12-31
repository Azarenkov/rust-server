use std::time::Duration;
use tokio::{sync::mpsc, task, time::sleep};
use crate::{adapters::{db::{db_connection::get_database, model::DbAdapter}, messaging::{fcm_adapter::FcmAdapter, fcm_connection::get_messaging_service}}, application::{new_data_service::interfaces::add_service_abstract::AddServiceAbstract, sync_service::{interfaces::sync_service_abstract::SyncServiceAbstract, sync_service::SyncService}}};
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


    let (new_tx, mut new_rx): (mpsc::Sender<(DbAdapter, String)>, mpsc::Receiver<(DbAdapter, String)>) = mpsc::channel(100);

    tokio::spawn(async move {
        while let Some(data) = new_rx.recv().await {
            let db = data.0;
            let token = data.1;
            if let Err(e) = db.add_new_data(&token).await {
                eprintln!("Error adding new data: {:?}", e);
            }
        }
    });

    web_server::get_web_server(db.clone(), new_tx).await?;
    Ok(())
}
