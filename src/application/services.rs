use std::f32::consts::E;
use std::vec;

use futures_util::stream::FuturesUnordered;
use mongodb::bson::{self, Document};
use mongodb::Collection;
use crate::adapters::api::client::ApiClient;
use crate::adapters::db::db_adapter::DbAdapter;
use crate::infrastructure::repositories::DbRepositoryAbstract;
use super::utils::errors::SyncError;

pub async fn sync_data_with_database(db: mongodb::Collection<bson::Document>) -> Result<(), SyncError> {

    match DbAdapter::get_users_tokens(db.clone()).await {
        Ok(tokens) => {
            for token in tokens {
                let api_client = ApiClient::new(&token, None, None);
                let response = api_client.get_user().await;

                match response {
                    Ok(user) => {
                        match DbAdapter::update_user_info(db.clone(), &token, user).await {
                            Ok(_) => {
                                println!("User info updated!");
                            },
                            Err(e) => {
                                println!("{:#?}", e);
                                return Err(SyncError::DatabaseError(e));
                            },
                        }
                    },
                    Err(e) => {
                        println!("{:#?}", e);
                        return Err(SyncError::ApiError(e));
                    },
                }
            }
            Ok(())
        },
        Err(e) => {
            println!("{:#?}", e);
            Err(SyncError::DatabaseError(e))
        }  
    }
}

pub async fn sync_courses_with_database(db: mongodb::Collection<bson::Document>) -> Result<(), SyncError> {
    let vectors = DbAdapter::get_tokens_and_ids(db.clone()).await?;

    for vector in vectors {

        let api_client = ApiClient::new(&vector.0, Some(vector.1), None);
        match api_client.get_courses().await {
            Ok(courses) => {
                DbAdapter::update_courses_info(db.clone(), &vector.0, courses).await?;
            },
            Err(e) => {
                println!("{:#?}", e);
                continue;
            },
        }
    }
    Ok(())
}

