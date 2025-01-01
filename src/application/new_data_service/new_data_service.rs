use async_trait::async_trait;

use crate::{adapters::db::{interfaces::token_repository_abstract::TokenRepositoryAbstract, model::DbAdapter}, application::utils::errors::SyncError};

use super::interfaces::{add_course_abstract::AddCourseAbstract, add_deadline_abstract::AddDeadlineAbstract, add_grade_abstract::AddGradeAbstact, add_service_abstract::AddServiceAbstract, add_user_data_abstract::AddUserDataAbstract};

#[async_trait]
impl AddServiceAbstract for DbAdapter {
    async fn add_new_data(&self, token: &String) -> Result<(), SyncError> {
        self.add_token(token).await?;
        self.add_user_data(token).await?;
        self.add_course(token).await?;
        self.add_deadline(token).await?;
        self.add_grade(token).await?;
        self.add_grade_overview(token).await?;
        Ok(())
    }
}