use std::collections::HashMap;

use tokio::sync::mpsc;

use crate::{adapters::{db::{interfaces::{grade_repository_abstract::GradeRepositoryAbstract, token_repository_abstract::TokenRepositoryAbstract}, model::DbAdapter}, http::http_client_repository::ApiClient, messaging::fcm_adapter::FcmAdapter, utils::errors::DbErrors}, application::{sync_service::{interfaces::sync_grades_abstract::SyncGradesWithDatabase, sync_service::SyncService}, utils::{errors::SyncError, helpers::tx_sender}}};

impl SyncGradesWithDatabase for SyncService {
    async fn sync_grades_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_userdid_and_courses().await?;

        for vector in vectors {
            let courses = vector.courses;
            let mut grades_data = Vec::new();
            let token = vector.token.unwrap_or_default();

            for course in courses {
                let api_client = ApiClient::new(&token, Some(vector.user_id.to_string()), Some(course.id.to_string()));
                let grades = api_client.get_grades().await?;
                grades.usergrades.clone().into_iter().for_each(|mut grade|{
                    grade.coursename = Some(course.fullname.clone());
                    grades_data.push(grade);
                });

            }

            match db.get_grades(&token).await {
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
                                        let device_token = db.get_device_token(&token).await;
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
                                    db.update_grades_info(&token, grades_data.clone()).await?
                                }
                            }
                        } else {
                            db.update_grades_info(&token, grades_data.clone()).await?
                        }
                    }
                },
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => db.update_grades_info(&token, grades_data).await?,
                        DbErrors::DbError(_error) => continue,
                    }
                },
            }
        }
        Ok(())
    }

    async fn sync_grades_overview_with_databse(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {
        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_userdid_and_courses().await?;

        for vector in vectors.iter() {
            let token = vector.token.clone().unwrap_or_default();
            let api_client = ApiClient::new(&token, None, None);
            let grades_overview = api_client.get_grades_overview().await?;
            let mut grades = grades_overview.grades;

            let courses = vector.courses.clone();

            for course in courses.iter() {
                for grade in grades.iter_mut() {
                    if grade.courseid == course.id {
                        grade.course_name = Some(course.fullname.clone())
                    }
                }
            }

            match db.get_grades_overview(&token).await {
                Ok(db_grades) => {
                    if grades == db_grades {
                        continue;
                    } else {
                        db.update_grades_overview(&token, &grades).await?
                    }
                    
                },
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => db.update_grades_overview(&token, &grades).await?,
                        DbErrors::DbError(_error) => continue,
                    }
                },
            }
        }

        Ok(())
    }
}