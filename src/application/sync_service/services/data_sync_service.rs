use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::{adapters::{db::{interfaces::{token_repository_abstract::TokenRepositoryAbstract, user_repository_abstract::UserRepositoryAbstract}, model::DbAdapter}, http::http_client_repository::ApiClient, messaging::fcm_adapter::FcmAdapter, utils::errors::DbErrors}, application::{sync_service::{interfaces::sync_data_abstract::SyncDataWithDatabase, sync_service::SyncService}, utils::{errors::SyncError, helpers::{compare_objects, tx_sender}}}};


#[async_trait]
impl SyncDataWithDatabase for SyncService {
    async fn sync_data_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        let tokens = db.get_users_tokens().await?;
        for token in tokens {
            let api_client = ApiClient::new(&token, None, None);
            let user = api_client.get_user().await?;

            match db.get_user_info(&token).await {
                Ok(user_info) => {
                        
                    if let Some(difference) = compare_objects(user.clone(), user_info.clone()) {
                        if let Some(ref tx) = tx {
                            match db.get_device_token(&token).await {
                                Ok(device_token) => {

                                        if let Some(username) = &difference.username {
                                            if let Some(fullname) = &difference.fullname {
                                                let title = "New User Data ".to_string();
                                                let body = format!("Email: {}\nFullname: {}", username, fullname);
                                                let message: FcmAdapter = FcmAdapter::new(&device_token, &title, &body, None);
                                                let tx_clone = tx.clone();

                                                tx_sender(message, tx_clone);
                                            }
                                        }
                                },
                                Err(_e) => (),
                            }
                        }
                        db.update_user_info(&token, user).await?;
                    }

                },
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => db.update_user_info(&token, user).await?,
                        DbErrors::DbError(_error) => continue,
                    }
                },
            }
        }
        Ok(())
    }
}