use crate::{adapters::{db::interfaces::{course_repository_abstract::CourseRepositoryAbstract, token_repository_abstract::TokenRepositoryAbstract}, http::http_client_repository::ApiClient}, application::{new_data_service::{interfaces::add_course_abstract::AddCourseAbstract, new_data_service::NewDataService}, utils::errors::SyncError}};

impl AddCourseAbstract for NewDataService {
    async fn add_course(&self, token: String) -> Result<(), SyncError> {
        let user_id = self.db.get_user_id(&token).await?;
        let api_client = ApiClient::new(&token, Some(user_id), None);
        let courses = api_client.get_courses().await?;
        self.db.update_courses_info(&token, courses).await?;
        Ok(())
    }
}