use async_trait::async_trait;

use crate::application::utils::errors::SyncError;

#[async_trait]
pub trait AddUserDataAbstract {
    async fn add_user_data(&self, token: &String) -> Result<(), SyncError>;
}