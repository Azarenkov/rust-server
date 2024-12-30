use crate::application::utils::errors::SyncError;

pub trait AddDeadlineAbstract {
    async fn add_deadline(&self, token: String) -> Result<(), SyncError>;
}