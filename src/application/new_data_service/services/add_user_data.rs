use async_trait::async_trait;

use crate::{adapters::{db::{interfaces::user_repository_abstract::UserRepositoryAbstract, model::DbAdapter}, http::http_client_repository::ApiClient}, application::{new_data_service::interfaces::add_user_data_abstract::AddUserDataAbstract, utils::errors::SyncError}};

#[async_trait]
impl AddUserDataAbstract for DbAdapter {
    async fn add_user_data(&self, token: &String) -> Result<(), SyncError> {
        let api_client = ApiClient::new(&token, None, None);
        let user = api_client.get_user().await?;
        self.update_user_info(&token, user).await?;
        
        Ok(())
    }
}