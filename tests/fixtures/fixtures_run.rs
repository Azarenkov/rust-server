use rust_server::adapters::db::interfaces::token_repository_abstract::TokenRepositoryAbstract;
use rust_server::adapters::db::interfaces::user_repository_abstract::UserRepositoryAbstract;
use rust_server::adapters::http_and_db_models::user::User;

use crate::utils::file_utils::read_from_file;
use crate::utils::db_utils::DbAdapterTest;

impl DbAdapterTest {
    pub async fn execute_imports(&self, token: &String) {
        
        let user = import_user_info_fixtures();
        
        if let Err(e) = self.db.add_token(&token).await {
            panic!("Failed to add token: {:?}", e);
        }
    
        if let Err(e) = self.db.update_user_info(&token, user).await {
            panic!("Failed to add user_info: {:?}", e);
        }
    }

    pub async fn delete_imports(&self, token: &String) {
        if let Err(e) = self.db.delete_document(&token).await {
            panic!("Failed to delete doc: {:?}", e);
        }
    }
}


fn import_user_info_fixtures() -> User {
    let user = read_from_file::<User>("tests/fixtures/user_info.json").unwrap();
    user
}
