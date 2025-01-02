use async_trait::async_trait;
use rust_server::adapters::db::interfaces::course_repository_abstract::CourseRepositoryAbstract;
use rust_server::adapters::db::model::DbAdapter;
use serde::de::DeserializeOwned;
use rust_server::adapters::db::interfaces::token_repository_abstract::TokenRepositoryAbstract;
use rust_server::adapters::db::interfaces::user_repository_abstract::UserRepositoryAbstract;
use rust_server::adapters::http_and_db_models::user::User;
use rust_server::adapters::http_and_db_models::course::Course;

// use crate::utils::db_utils::DbAdapterTest;

#[async_trait]
pub trait Importable: DeserializeOwned {
    fn file_path() -> &'static str;
    async fn import(db: &DbAdapter, token: &String, data: Self) -> Result<(), mongodb::error::Error>;
}

#[async_trait]
impl Importable for User {
    fn file_path() -> &'static str {
        "tests/fixtures/user_info.json"
    }

    async fn import(db: &DbAdapter, token: &String, data: Self) -> Result<(), mongodb::error::Error> {
        db.add_token(&token).await?;
        db.update_user_info(&token, data).await?;
        Ok(())
    }
}

#[async_trait]
impl Importable for Vec<Course> {
    fn file_path() -> &'static str {
        "tests/fixtures/user_courses.json"
    }

    async fn import(db: &DbAdapter, token: &String, data: Self) -> Result<(), mongodb::error::Error> {
        db.add_token(&token).await?;
        db.update_courses_info(&token, data).await?;
        Ok(())
    }
}