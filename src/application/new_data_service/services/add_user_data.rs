use crate::{adapters::{db::interfaces::user_repository_abstract::UserRepositoryAbstract, http::http_client_repository::ApiClient}, application::{new_data_service::{interfaces::add_user_data_abstract::AddUserDataAbstract, new_data_service::NewDataService}, utils::errors::SyncError}};

impl AddUserDataAbstract for NewDataService {
    async fn add_user_data(&self, token: String) -> Result<(), SyncError> {
        let api_client = ApiClient::new(&token, None, None);
        let user = api_client.get_user().await?;
        self.db.update_user_info(&token, user).await?;
        
        Ok(())
    }
}