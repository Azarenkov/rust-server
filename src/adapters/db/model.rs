use mongodb::{bson::Document, Collection};

pub struct DbAdapter {
    pub collection: Collection<Document>,
}

impl DbAdapter {
    pub fn new(collection: Collection<Document>) -> Self {
        DbAdapter { collection }
    }
}