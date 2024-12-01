use mongodb::bson::{self};
use serde_json::{from_str, to_string, Value};
use crate::adapters::api::client::ApiClient;
use crate::adapters::db::db_adapter::DbAdapter;
use crate::application::repositories::sync_service_abstract::SyncServiceAbstract;
use crate::application::utils::helpers::{compare, extract_link_and_date};
use crate::infrastructure::repositories::db_repository_abstract::DbRepositoryAbstract;
use crate::application::utils::errors::SyncError;
use chrono::Utc;
use crate::adapters::utils::errors::DbErrors;

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
        let tokens = db.get_users_tokens().await?;
        for token in tokens {
            let api_client = ApiClient::new(&token, None, None);
            let user = api_client.get_user().await?;

            match db.get_user_info(&token).await {
                Ok(user_info) => {
                    let user_value = serde_json::to_string(&user).map_err(|e| SyncError::SerdeError(e))?;
                    let user_db_value = serde_json::to_string(&user_info).map_err(|e| SyncError::SerdeError(e))?;
                    
                    let comparing = compare(user_value, user_db_value);

                    match comparing {
                        true => (),
                        false => db.update_user_info(&token, user).await?,                        
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

    async fn sync_courses_with_database(&self) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_ids().await?;

        for vector in vectors {
    
            let api_client = ApiClient::new(&vector.0, Some(vector.1), None);
            let courses = api_client.get_courses().await?;

            match db.get_courses(&vector.0).await {
                Ok(db_courses) => {
                    let courses_value = serde_json::to_string(&courses).map_err(|e| SyncError::SerdeError(e))?;
                    let db_courses_value = serde_json::to_string(&db_courses).map_err(|e| SyncError::SerdeError(e))?;

                    let comparing = compare(courses_value, db_courses_value);

                    match comparing {
                        true => (),
                        false => db.update_courses_info(&vector.0, courses).await?,
                    }
                },
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => db.update_courses_info(&vector.0, courses).await?,
                        DbErrors::DbError(_error) => continue,
                    }
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
                let grades = api_client.get_grades().await?;

                grades.usergrades.clone().into_iter().for_each(|mut grade|{
                    grade.coursename = Some(course.fullname.clone());
                    grades_data.push(grade);
                });

            }

            match db.get_grades(&vector.token).await {
                Ok(db_grades) => {
                    let grades_value = serde_json::to_string(&grades_data).map_err(|e| SyncError::SerdeError(e))?;
                    let db_grades_value = serde_json::to_string(&db_grades).map_err(|e| SyncError::SerdeError(e))?;

                    let comparing = compare(grades_value, db_grades_value);

                    match comparing {
                        true => (),
                        false => db.update_grades_info(&vector.token, grades_data).await?,
                    }
                },
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => db.update_grades_info(&vector.token, grades_data).await?,
                        DbErrors::DbError(_error) => continue,
                    }
                },
            }
        }
        Ok(())
    }
    
    async fn sync_deadlines_with_database(&self) -> Result<(), SyncError> {

        let db = DbAdapter::new(self.db.clone());
        let tokens = db.get_users_tokens().await?;

        for token in tokens {

            let api_client = ApiClient::new(&token, None, None);
            let mut deadlines_data = Vec::new();

            let deadlines = api_client.get_deadlines().await?;

            deadlines.events.clone().into_iter().for_each(|mut deadline|{
                let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east(6 * 3600));
                let current_unix_time = current_time.timestamp();

                if (deadline.timeusermidnight + 3600) > current_unix_time.try_into().unwrap() {
                    let time_description= extract_link_and_date(&deadline.formattedtime);
                    deadline.formattedtime = time_description.unwrap_or_else(|| "No time".to_string());                            
                    deadlines_data.push(deadline);
                }
            });

            match db.get_deadlines(&token).await {
                Ok(db_deadlines) => {
                    let deadlines_value = serde_json::to_string(&deadlines_data).map_err(|e| SyncError::SerdeError(e))?;
                    let db_deadlines_value = serde_json::to_string(&db_deadlines).map_err(|e| SyncError::SerdeError(e))?;

                    let comparing = compare(deadlines_value, db_deadlines_value);

                    match comparing {
                        true => (),
                        false => db.update_deadline_info(&token, deadlines_data).await?,
                    }
                },
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => db.update_deadline_info(&token, deadlines_data).await?,
                        DbErrors::DbError(_error) => continue,
                    }
                },
            }
        }
        Ok(())
    }
    
    async fn sync_all_data(&self) -> Result<(), SyncError> {
        
        self.sync_data_with_database().await?;
        self.sync_courses_with_database().await?;
        self.sync_grades_with_database().await?;
        self.sync_deadlines_with_database().await?;

        Ok(())
    }
}

