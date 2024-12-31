use crate::{adapters::{db::{interfaces::{course_repository_abstract::CourseRepositoryAbstract, token_repository_abstract::TokenRepositoryAbstract}, model::DbAdapter}, http::http_client_repository::ApiClient}, application::{new_data_service::interfaces::add_course_abstract::AddCourseAbstract, utils::errors::SyncError}};

impl AddCourseAbstract for DbAdapter {
    async fn add_course(&self, token: &String) -> Result<(), SyncError> {
        let user_id = self.get_user_id(&token).await?;
        let api_client = ApiClient::new(&token, Some(user_id), None);
        let courses = api_client.get_courses().await?;

        self.update_courses_info(&token, courses).await?;
        Ok(())
    }
}