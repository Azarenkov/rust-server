use crate::application::utils::errors::SyncError;

pub trait AddCourseAbstract {
    async fn add_course(&self, token: &String) ->  Result<(), SyncError>;
}