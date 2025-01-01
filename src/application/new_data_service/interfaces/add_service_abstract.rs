use async_trait::async_trait;

use crate::application::utils::errors::SyncError;

#[async_trait]
pub trait AddServiceAbstract {
    async fn add_new_data(&self, token: &String) -> Result<(), SyncError>;
}