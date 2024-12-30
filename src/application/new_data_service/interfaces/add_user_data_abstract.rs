use crate::application::utils::errors::SyncError;

pub trait AddUserDataAbstract {
    async fn add_user_data(&self, token: String) -> Result<(), SyncError>;
}