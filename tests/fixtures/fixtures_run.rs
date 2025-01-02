use rust_server::adapters::db::interfaces::token_repository_abstract::TokenRepositoryAbstract;

use crate::utils::file_utils::read_from_file;
use crate::utils::db_utils::DbAdapterTest;
use crate::utils::import_utils::Importable;

impl DbAdapterTest {
    pub async fn execute_imports<T: Importable>(&self, token: &String) {
        let data = read_from_file::<T>(T::file_path()).unwrap();
        if let Err(e) = T::import(self, token, data).await {
            panic!("Failed to import data: {:?}", e);
        }
    }

    pub async fn delete_imports(&self, token: &String) {
        if let Err(e) = self.db.delete_document(&token).await {
            panic!("Failed to delete doc: {:?}", e);
        }
    }
}



