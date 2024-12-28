use tokio::sync::mpsc;

use crate::{adapters::{db::{interfaces::{course_repository_abstract::CourseRepositoryAbstract, token_repository_abstract::TokenRepositoryAbstract}, model::DbAdapter}, http::http_client_repository::ApiClient, messaging::fcm_adapter::FcmAdapter, utils::errors::DbErrors}, application::{sync_service::{interfaces::sync_courses_abstract::SyncCoursesWithDatabase, sync_service::SyncService}, utils::{errors::SyncError, helpers::tx_sender}}};

impl SyncCoursesWithDatabase for SyncService {
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
}