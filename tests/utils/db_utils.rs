use mongodb::bson::Document;
use mongodb::Collection;
use rust_server::adapters::db::model::DbAdapter;

#[derive(Clone)]
pub struct DbAdapterTest {
    pub db: DbAdapter
}

impl DbAdapterTest {
    pub async fn new(collection: &Collection<Document>) -> Self {
        let db = DbAdapter::new(collection.clone());
        DbAdapterTest { db }
    }
}

