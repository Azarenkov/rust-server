use async_trait::async_trait;

use crate::application::utils::errors::SyncError;

#[async_trait]
pub trait AddDeadlineAbstract {
    async fn add_deadline(&self, token: &String) -> Result<(), SyncError>;
}