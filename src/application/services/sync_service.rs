use mongodb::bson::{self};
use crate::adapters::api::client::ApiClient;
use crate::adapters::db;
use crate::adapters::db::db_adapter::{self, DbAdapter};
use crate::application::repositories::sync_service_abstract::SyncServiceAbstract;
use crate::infrastructure::repositories::db_repository_abstract::DbRepositoryAbstract;
use crate::application::utils::errors::SyncError;

pub struct SyncService {
    pub db: mongodb::Collection<bson::Document>,
}

impl SyncService {
    pub fn new(db: mongodb::Collection<bson::Document>) -> Self {
        SyncService { db }
    }
}

impl SyncServiceAbstract for SyncService {
    async fn sync_data_with_database(&self) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        match db.get_users_tokens().await {
            Ok(tokens) => {
                for token in tokens {
                    let api_client = ApiClient::new(&token, None, None);
                    let response = api_client.get_user().await;
    
                    match response {
                        Ok(user) => {
                            match db.update_user_info(&token, user).await {
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

    async fn sync_courses_with_database(&self) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_ids().await?;

        for vector in vectors {
    
            let api_client = ApiClient::new(&vector.0, Some(vector.1), None);
            match api_client.get_courses().await {
                Ok(courses) => {
                    db.update_courses_info(&vector.0, courses).await?;
                },
                Err(e) => {
                    println!("{:#?}", e);
                    return Err(SyncError::ApiError(e))
                },
            }
        }
        Ok(())
    }
    
    async fn sync_grades_with_database(&self) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_userdid_and_courses().await?;

        for vector in vectors {
            let courses = vector.courses;
            let mut grades_data = Vec::new();

            for course in courses {
                let api_client = ApiClient::new(&vector.token, Some(vector.user_id.to_string()), Some(course.id.to_string()));
                match api_client.get_grades().await {
                    Ok(grades) => {
                        grades.usergrades.clone().into_iter().for_each(|mut grade|{
                            grade.coursename = Some(course.fullname.clone());
                            grades_data.push(grade);
                        });
                        
                    },
                    Err(e) => {
                        println!("{:#?}", e);
                        return Err(SyncError::ApiError(e))
                    },
                }
            }
            db.update_grades_info(&vector.token, grades_data).await?;
        }
        // println!("{:#?}", grades_data);
        Ok(())
    }
}

