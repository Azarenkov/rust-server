use std::collections::HashMap;

use mongodb::bson::{self};
use tokio::sync::mpsc;
use crate::adapters::api::client::ApiClient;
use crate::adapters::db::db_adapter::DbAdapter;
use crate::adapters::messaging::fcm_adapter::FcmAdapter;
use crate::application::repositories::sync_service_abstract::SyncServiceAbstract;
use crate::application::utils::helpers::{extract_date_and_time, extract_time, parse_time_to_seconds, tx_sender};
use crate::domain::utils::compare_objects;
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

    async fn sync_courses_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_ids().await?;

        for vector in vectors {
    
            let api_client = ApiClient::new(&vector.0, Some(vector.1), None);
            let courses = api_client.get_courses().await?;

            match db.get_courses(&vector.0).await {
                Ok(db_courses) => {

                    for course in courses.iter() {
                        if !db_courses.iter().any(|db_course| db_course.fullname == course.fullname) {
                            if let Some(ref tx) = tx {
                                match db.get_device_token(&vector.0).await {
                                    Ok(device_token) => {
                                        let title = "New Course".to_string();
                                        let body = &course.fullname;
                                        let message: FcmAdapter = FcmAdapter::new(&device_token, &title, &body, None);
                                        let tx_clone = tx.clone();
                                        tx_sender(message, tx_clone);

                                    },
                                    Err(_e) => (),
                                }
                            }
                            db.update_courses_info(&vector.0, courses.clone()).await?;                     
                        }
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
    
    async fn sync_grades_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {
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
                    
                    let mut grades_map_new = HashMap::new();
                    let mut grades_map_old = HashMap::new();
                    
                    for grade in grades_data.iter() {
                        grades_map_new.insert(grade.coursename.clone().unwrap_or_default(), grade.gradeitems.clone());
                    }

                    for grade in db_grades.iter() {
                        grades_map_old.insert(grade.coursename.clone().unwrap_or_default(), grade.gradeitems.clone());
                    }
                    
                    for (i, j) in grades_map_new.iter() {
                        if let Some(value) = grades_map_old.get_key_value(i) {
                            for (m, k) in j.iter().zip(value.1) {
                                if m != k {
                                    if let Some(ref tx) = tx {
                                        let device_token = db.get_device_token(&vector.token).await;
                                        match device_token {
                                            Ok(device_token) => {
                                                let title = format!("{}", m.itemname);
                                                let body = &m.percentageformatted;
                                                let old_body = format!("{}\n{} ->", i, &k.percentageformatted);
                                                let message: FcmAdapter = FcmAdapter::new(&device_token, &title, &body, Some(&old_body));
                                                let tx_clone = tx.clone();
                                                tx_sender(message, tx_clone);
                                            },
                                            Err(_e) => (),
                                        }
                                    }
                                    db.update_grades_info(&vector.token, grades_data.clone()).await?
                                }
                            }
                        } else {
                            db.update_grades_info(&vector.token, grades_data.clone()).await?
                        }
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
    
    async fn sync_deadlines_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {

        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_userdid_and_courses().await?;

        let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east(6 * 3600));
        let current_unix_time = current_time.timestamp();

        for vector in vectors {
            let courses = vector.courses;
            let mut deadlines_data = Vec::new();

            for course in courses {
                let api_client = ApiClient::new(&vector.token, Some(vector.user_id.to_string()), Some(course.id.to_string()));
                let deadlines = api_client.get_deadlines().await?;
                deadlines.events.clone().into_iter().for_each(|mut deadline|{
                    deadline.coursename = Some(course.fullname.clone());

                    let seconds_after_mid;

                    if let Some(time_str) = extract_time(&deadline.formattedtime) {
                        match parse_time_to_seconds(&time_str) {
                            Ok(seconds) => seconds_after_mid = seconds,
                            Err(_e) => seconds_after_mid = 0,
                        }
                    } else {
                        seconds_after_mid = 0;
                    }

                    if deadline.timeusermidnight + seconds_after_mid >  current_unix_time.try_into().unwrap() {
                        let time_description= extract_date_and_time(&deadline.formattedtime);
                        deadline.formattedtime = time_description.unwrap_or_else(|| "No time".to_string());                            
                        deadlines_data.push(deadline.clone());
                    }

                });
            }

            match db.get_deadlines(&vector.token).await {
                Ok(db_deadlines) => {
                    if let Some(db_deadlines) = db_deadlines {
                        for (deadline, db_deadline) in deadlines_data.iter().zip(db_deadlines.iter()) {
                            if deadline != db_deadline {
                                if let Some(ref tx) = tx {
                                    let device_token = db.get_device_token(&vector.token).await;
                                    match device_token {
                                        Ok(device_token) => {
                                            let title = format!("{}", deadline.coursename.clone().unwrap_or_default());
                                            let body = format!("{}\n{}", deadline.name, deadline.formattedtime);
                                            let message: FcmAdapter = FcmAdapter::new(&device_token, &title, &body, None);
                                            let tx_clone = tx.clone();
                                            tx_sender(message, tx_clone);
                                        },
                                        Err(_e) => (),
                                    }
                                }
                                db.update_deadline_info(&vector.token, deadlines_data.clone()).await?;
                                return Ok(());
                            } else if db_deadline.clone().name.is_empty() {
                                println!("Empty");
                            }
                        }
                    } else {
                        db.update_deadline_info(&vector.token, deadlines_data.clone()).await?;
                    };
                },
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => db.update_deadline_info(&vector.token, deadlines_data).await?,
                        DbErrors::DbError(_error) => continue,
                    }
                },
            }

        }

        Ok(())
    }
    
    async fn sync_all_data(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {
        
        self.sync_data_with_database(tx.clone()).await?;
        self.sync_courses_with_database(tx.clone()).await?;
        self.sync_grades_with_database(tx.clone()).await?;
        self.sync_deadlines_with_database(tx.clone()).await?;

        Ok(())
    }
}

