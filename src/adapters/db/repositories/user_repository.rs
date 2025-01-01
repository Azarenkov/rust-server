use crate::adapters::{db::{interfaces::user_repository_abstract::UserRepositoryAbstract, model::DbAdapter}, http_and_db_models::user::User, utils::errors::DbErrors};
use async_trait::async_trait;
use futures_util::TryStreamExt;
use mongodb::{bson::{self, doc, Bson}, error::Error as mongodbErr};

#[async_trait]
impl UserRepositoryAbstract for DbAdapter {
    async fn get_users_tokens(&self) -> Result<Vec<String>, mongodbErr> {
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

    async fn get_user_info(&self, token: &String) -> Result<User, DbErrors> {
        let document = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| {
            DbErrors::DbError(e)
        })?;

        match document {
            Some(doc) => {
                if let Some(user_info) = doc.get_document("user_info").ok() {
                    match bson::from_bson::<User>(Bson::Document(user_info.clone())) {
                        Ok(user) => Ok(user),
                        Err(e) => {
                            eprintln!("Deserialization error: {:?}", e);
                            Err(DbErrors::DbError(e.into()))
                        }
                    }
                } else {
                    Err(DbErrors::NotFound())
                }
            },
            None => Err(DbErrors::NotFound()),
        }
    }

    async fn update_user_info(&self, token: &String, user: User) -> Result<(), mongodbErr> {
        self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"user_info": bson::to_bson(&user).unwrap()}
            },
            None
        ).await?;
        println!("User_info updated!");

        Ok(())
    }
} 