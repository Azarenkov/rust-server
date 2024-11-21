use mongodb::bson;
use crate::api::client::ApiClient;
use crate::infrastructure::repositories::update_user_info;
use super::utils::errors::SyncError;

pub async fn sync_data_with_database(db: mongodb::Collection<bson::Document> ,token: &String) -> Result<(), SyncError> {
    let api_client = ApiClient::new(token, None, None);

    match api_client.get_user().await {
        Ok(user) => {
            match update_user_info(db, token, user).await {
                Ok(_) => {
                    println!("User info updated!");
                    Ok(())
                },
                Err(e) => {
                    println!("{:#?}", e);
                    Err(SyncError::DatabaseError(e))
                },
            }
        },
        Err(e) => {
            println!("{:#?}", e);
            Err(SyncError::ApiError(e))
        },
    }

}