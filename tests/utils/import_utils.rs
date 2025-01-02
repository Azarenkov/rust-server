use async_trait::async_trait;
use rust_server::adapters::db::interfaces::course_repository_abstract::CourseRepositoryAbstract;
use rust_server::adapters::db::interfaces::deadline_repository_abstract::DeadlineRepositoryAbstract;
use rust_server::adapters::db::interfaces::grade_repository_abstract::GradeRepositoryAbstract;
use rust_server::adapters::db::model::DbAdapter;
use rust_server::adapters::http_and_db_models::deadline::Deadline;
use rust_server::adapters::http_and_db_models::grade::GradeItems;
use rust_server::adapters::http_and_db_models::grade_overview::GradeOverview;
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

#[async_trait]
impl Importable for Vec<GradeItems> {
    fn file_path() -> &'static str {
        "tests/fixtures/user_grades.json"
    }

    async fn import(db: &DbAdapter, token: &String, data: Self) -> Result<(), mongodb::error::Error> {
        db.add_token(&token).await?;
        db.update_grades_info(&token, data).await?;
        Ok(())
    }
}

#[async_trait]
impl Importable for Vec<GradeOverview> {
    fn file_path() -> &'static str {
        "tests/fixtures/user_grades_overview.json"
    }

    async fn import(db: &DbAdapter, token: &String, data: Self) -> Result<(), mongodb::error::Error> {
        db.add_token(&token).await?;
        db.update_grades_overview(&token, &data).await?;
        Ok(())
    }
}

#[async_trait]
impl Importable for Vec<Deadline> {
    fn file_path() -> &'static str {
        "tests/fixtures/user_deadlines.json"
    }

    async fn import(db: &DbAdapter, token: &String, data: Self) -> Result<(), mongodb::error::Error> {
        db.add_token(&token).await?;
        db.update_deadline_info(&token, data).await?;
        Ok(())
    }
}