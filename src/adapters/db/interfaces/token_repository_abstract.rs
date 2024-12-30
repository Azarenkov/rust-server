use mongodb::error::Error as mongodbErr;
use crate::adapters::{http_and_db_models::db_user_course_info::UserCourseInfo, utils::errors::DbErrors};

pub trait TokenRepositoryAbstract {
    async fn get_tokens_and_ids(&self) -> Result<Vec<(String, String)>, mongodbErr>;
    async fn get_tokens_and_userdid_and_courses(&self) -> Result<Vec<UserCourseInfo>, mongodbErr>;
    async fn get_device_token(&self, token: &String) -> Result<String, DbErrors>;
    async fn add_token(&self, token: &String) -> Result<(), mongodbErr>;
    async fn find_token(&self, token: &String) -> Result<(), DbErrors>;
    async fn add_device_token(&self, token: &String, device_token: &String) -> Result<(), mongodbErr>;
    async fn delete_document(&self, token: &String) -> Result<(), mongodbErr>;
    async fn get_user_id(&self, token: &String) -> Result<String, mongodbErr>;
    async fn get_user_id_and_courses_id(&self, token: &String) -> Result<UserCourseInfo, mongodbErr>;
}