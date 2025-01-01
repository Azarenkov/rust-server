use async_trait::async_trait;
use chrono::Utc;
use tokio::sync::mpsc;

use crate::{adapters::{db::{interfaces::{deadline_repository_abstract::DeadlineRepositoryAbstract, token_repository_abstract::TokenRepositoryAbstract}, model::DbAdapter}, http::http_client_repository::ApiClient, messaging::fcm_adapter::FcmAdapter, utils::errors::DbErrors}, application::{sync_service::{interfaces::sync_deadlines_abstract::SyncDeadlinesWithDatabase, sync_service::SyncService}, utils::{errors::SyncError, helpers::{extract_date_and_time, extract_time, parse_time_to_seconds, tx_sender}}}};

#[async_trait]
impl SyncDeadlinesWithDatabase for SyncService {
    async fn sync_deadlines_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {

        let db = DbAdapter::new(self.db.clone());
        let vectors = db.get_tokens_and_userdid_and_courses().await?;

        let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(6 * 3600).unwrap());
        let current_unix_time = current_time.timestamp();

        for vector in vectors {
            let courses = vector.courses;
            let mut deadlines_data = Vec::new();
            let token = vector.token.unwrap_or_default();
            for course in courses {
                let api_client = ApiClient::new(&token, Some(vector.user_id.to_string()), Some(course.id.to_string()));
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

            match db.get_deadlines(&token).await {
                Ok(db_deadlines) => {
                    if let Some(db_deadlines) = db_deadlines {
                        for (deadline, db_deadline) in deadlines_data.iter().zip(db_deadlines.iter()) {
                            if deadline != db_deadline {
                                if let Some(ref tx) = tx {
                                    let device_token = db.get_device_token(&token).await;
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
                                db.update_deadline_info(&token, deadlines_data.clone()).await?;
                                return Ok(());
                            } else if db_deadline.clone().name.is_empty() {
                                println!("Empty");
                            }
                        }
                    } else {
                        db.update_deadline_info(&token, deadlines_data.clone()).await?;
                    };
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
}