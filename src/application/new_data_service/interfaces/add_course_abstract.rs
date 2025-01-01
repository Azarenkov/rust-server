use async_trait::async_trait;

use crate::application::utils::errors::SyncError;

#[async_trait]
pub trait AddCourseAbstract {
    async fn add_course(&self, token: &String) ->  Result<(), SyncError>;
}