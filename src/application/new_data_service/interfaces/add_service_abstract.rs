use crate::application::utils::errors::SyncError;

pub trait AddServiceAbstract {
    async fn add_new_data(&self, token: &String) -> Result<(), SyncError>;
}