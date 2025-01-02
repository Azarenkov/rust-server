use async_trait::async_trait;
use rust_server::adapters::db::interfaces::token_repository_abstract::TokenRepositoryAbstract;
use rust_server::adapters::db::model::DbAdapter;
use crate::utils::file_utils::read_from_file;
use crate::utils::import_utils::Importable;

#[async_trait]
pub trait DbAdabterTestAbstract {
    async fn execute_imports<T: Importable>(&self, token: &String);
    async fn delete_imports(&self, token: &String);
}

#[async_trait]
impl DbAdabterTestAbstract for DbAdapter {
    async fn execute_imports<T: Importable>(&self, token: &String) {
        let data = read_from_file::<T>(T::file_path()).unwrap();
        if let Err(e) = T::import(self, token, data).await {
            panic!("Failed to import data: {:?}", e);
        }
    }

    async fn delete_imports(&self, token: &String) {
        if let Err(e) = self.delete_document(&token).await {
            panic!("Failed to delete doc: {:?}", e);
        }
    }
}



