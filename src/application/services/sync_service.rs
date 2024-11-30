use mongodb::bson::{self};
use serde_json::{from_str, to_string, Value};
use crate::adapters::api::client::ApiClient;
use crate::adapters::db::db_adapter::DbAdapter;
use crate::application::repositories::sync_service_abstract::SyncServiceAbstract;
use crate::application::utils::helpers::extract_link_and_date;
use crate::infrastructure::repositories::db_repository_abstract::DbRepositoryAbstract;
use crate::application::utils::errors::SyncError;
use chrono::Utc;

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
                            match db.get_user_info(&token).await {
                                Ok(user_info) => {
                                    let user_json = serde_json::to_string(&user);
                                    let user_info_json = serde_json::to_string(&user_info);

                                    // println!("{:?}", user_json);
                                    // println!("{:?}", user_info_json);  

                                    match user_json {
                                        Ok(user_value) => {
                                            match user_info_json {
                                                Ok(user_info_value) => {

                                                    if user_value != user_info_value {

                                                        match db.update_user_info(&token, user).await {
                                                            Ok(_) => {
                                                                println!("User info updated!");
                                                            },
                                                            Err(e) => {
                                                                println!("{:#?}", e);
                                                                return Err(SyncError::DatabaseError(e));
                                                            },
                                                        }
                                                    }
                                                },
                                                Err(e) => return Err(SyncError::SerdeError(e)),
                                            }
                                        },
                                        Err(e) => return Err(SyncError::SerdeError(e)),
                                    }           
                                   
 
                                },
                                Err(e) => {
                                    match e {
                                        crate::adapters::utils::errors::DbErrors::NotFound() => {
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
                                        crate::adapters::utils::errors::DbErrors::DbError(error) => return Err(SyncError::DatabaseError(error)),
                                    }
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
                    match db.get_courses(&vector.0).await {
                        Ok(db_courses) => {
                            let courses_json = serde_json::to_string(&courses);
                            let db_courses_json = serde_json::to_string(&db_courses);
                            // println!("{:#?}", courses_json);
                            // println!("{}", db_courses_json);

                            match courses_json {
                                Ok(courses_value) => {
                                    match db_courses_json {
                                        Ok(db_courses_value) => {
                                            if courses_value != db_courses_value {
                                                match db.update_courses_info(&vector.0, courses).await {
                                                    Ok(_) => {
                                                        println!("Courses info updated!");
                                                    },
                                                    Err(e) => {
                                                        println!("{:#?}", e);
                                                        return Err(SyncError::DatabaseError(e));
                                                    },
                                                }
                                            }
                                        },
                                        Err(e) => return Err(SyncError::SerdeError(e)),
                                    }
                                },
                                Err(e) => return Err(SyncError::SerdeError(e)),
                            }
                        },
                        Err(e) => {
                            match e {
                                crate::adapters::utils::errors::DbErrors::NotFound() => {
                                    match db.update_courses_info(&vector.0, courses).await {
                                        Ok(_) => {
                                            println!("Courses info updated!");
                                        },
                                        Err(e) => {
                                            println!("{:#?}", e);
                                            return Err(SyncError::DatabaseError(e));
                                        },
                                    }
                                },
                                crate::adapters::utils::errors::DbErrors::DbError(error) => return Err(SyncError::DatabaseError(error)),
                            }
                        },
                    }
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
        println!("Grades updated!");

        Ok(())
    }
    
    async fn sync_deadlines_with_database(&self) -> Result<(), SyncError> {

        let db = DbAdapter::new(self.db.clone());
        let tokens = db.get_users_tokens().await?;

        for token in tokens {

            let api_client = ApiClient::new(&token, None, None);
            let mut deadlines_data = Vec::new();

            match api_client.get_deadlines().await {
                Ok(deadlines) => {
                    deadlines.events.clone().into_iter().for_each(|mut deadline|{
                        let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east(6 * 3600));
                        let current_unix_time = current_time.timestamp();

                        if (deadline.timeusermidnight + 3600) > current_unix_time.try_into().unwrap() {
                            let time_description= extract_link_and_date(&deadline.formattedtime);
                            deadline.formattedtime = time_description.unwrap_or_else(|| "No time".to_string());                            
                            deadlines_data.push(deadline);
                        }

                    });
                },
                Err(e) => return Err(SyncError::ApiError(e)),
            }
            db.update_deadline_info(&token, deadlines_data).await?;
        }
        println!("Deadlines updated!");

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

