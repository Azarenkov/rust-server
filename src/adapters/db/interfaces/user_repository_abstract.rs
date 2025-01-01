use async_trait::async_trait;
use mongodb::error::Error as mongodbErr;

use crate::adapters::{http_and_db_models::user::User, utils::errors::DbErrors};

#[async_trait]
pub trait UserRepositoryAbstract {
    async fn get_users_tokens(&self) -> Result<Vec<String>, mongodbErr>;
    async fn get_user_info(&self, token: &String) -> Result<User, DbErrors>;
    async fn update_user_info(&self, token: &String, user: User) -> Result<(), mongodbErr>;
}