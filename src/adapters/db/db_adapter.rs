

use futures_util::TryStreamExt;
use mongodb::bson::{self, doc, Document};
use mongodb::Collection;
use crate::domain::course::Course;
use crate::domain::user::User;
use crate::infrastructure::repositories::db_repository_abstract::DbRepositoryAbstract;


pub struct DbAdapter {
    pub collection: Collection<Document>,
}

impl DbAdapter {
    pub fn new(collection: Collection<Document>) -> Self {
        DbAdapter { collection }
    }
}

impl DbRepositoryAbstract for DbAdapter {
    async fn update_user_info(&self, token: &String, user: User) -> Result<(), mongodb::error::Error> {
        match self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"user_info": bson::to_bson(&user).unwrap()}
            },
            None
        ).await {
            Ok(_) => Ok(()),
        Err(e) =>  Err(e),
        }
    }
    
    async fn get_users_tokens(&self) -> Result<Vec<String>, mongodb::error::Error> {
        let mut tokens: Vec<String> = Vec::new();
        let filter = doc! {"token": {"$exists": true}};
        let mut cursor = self.collection.find(filter, None).await?;
        while let Some(doc) = cursor.try_next().await? {
            if let Some(token) = doc.get_str("token").ok() {
                tokens.push(token.to_string());
            }
        }
        Ok(tokens)
    }
    
    async fn get_tokens_and_ids(&self) -> Result<Vec<(String, String)>, mongodb::error::Error> {
        let mut tokens_and_ids: Vec<(String, String)> = Vec::new();
        let filter = doc! {"token": {"$exists": true}, "user_info": {"$exists": true}};
        let mut cursor = self.collection.find(filter, None).await?;
        while let Some(doc) = cursor.try_next().await? {
            if let Some(token) = doc.get_str("token").ok() {
                if let Some(user_info) = doc.get_document("user_info").ok() {
                    if let Some(id) = user_info.get_i64("userid").ok() {
                        tokens_and_ids.push((token.to_string(), id.to_string()));
                    }
                }
            }
        }
        Ok(tokens_and_ids)
    }
    
    async fn update_courses_info(&self, token: &String, courses: Vec<Course>) -> Result<(), mongodb::error::Error> {
        match self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"courses": bson::to_bson(&courses).unwrap()}
            },
            None
        ).await {
            Ok(_) => Ok(()),
        Err(e) =>  Err(e),
        }
    }

    
}


// pub async fn update_user_info(db: mongodb::Collection<bson::Document>, token: &String, user: User) -> Result<(), mongodb::error::Error> {
//     match db.update_one(
//         bson::doc! {"token": token},
//         bson::doc! {
//             "$set": {"user_info": bson::to_bson(&user).unwrap()}
//         },
//         None
//     ).await {
//         Ok(_) => {
//             Ok(())
//         },
//         Err(e) => {
//             Err(e)
//         },
//     }
// }