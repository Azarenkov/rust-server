use chrono::Utc;

use crate::{adapters::{db::{interfaces::{deadline_repository_abstract::DeadlineRepositoryAbstract, token_repository_abstract::TokenRepositoryAbstract}, model::DbAdapter}, http::http_client_repository::ApiClient}, application::{new_data_service::interfaces::add_deadline_abstract::AddDeadlineAbstract, utils::{errors::SyncError, helpers::{extract_date_and_time, extract_time, parse_time_to_seconds}}}};

impl AddDeadlineAbstract for DbAdapter {
    async fn add_deadline(&self, token: &String) -> Result<(), SyncError> {
        let user_data = self.get_user_id_and_courses_id(&token).await?;

        let current_time = Utc::now().with_timezone(&chrono::FixedOffset::east_opt(6 * 3600).unwrap());
        let current_unix_time = current_time.timestamp();
        
        let mut deadlines_data = Vec::new();

        for course in user_data.courses {
            let api_client = ApiClient::new(&token, Some(user_data.user_id.to_string()), Some(course.id.to_string()));
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

        self.update_deadline_info(&token, deadlines_data).await?;
        Ok(())
    }
}