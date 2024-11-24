use crate::application::utils::errors::SyncError;

pub trait SyncServiceAbstract {
    async fn sync_data_with_database(&self) -> Result<(), SyncError>;
    async fn sync_courses_with_database(&self) -> Result<(), SyncError>;
    async fn sync_grades_with_database(&self) -> Result<(), SyncError>;
}