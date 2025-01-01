use async_trait::async_trait;
use mongodb::error::Error as mongodbErr;
use crate::adapters::{http_and_db_models::course::Course, utils::errors::DbErrors};

#[async_trait]
pub trait CourseRepositoryAbstract {
    async fn get_courses(&self, token: &String) -> Result<Vec<Course>, DbErrors>;
    async fn update_courses_info(&self, token: &String, courses: Vec<Course>) -> Result<(), mongodbErr>;
}