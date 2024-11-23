use mongodb::bson;
use crate::domain::user::User;
use crate::domain::course::Course;

pub trait DbRepositoryAbstract {
    async fn get_users_tokens(db: mongodb::Collection<bson::Document>) -> Result<Vec<String>, mongodb::error::Error>;
    async fn update_user_info(db: mongodb::Collection<bson::Document>, token: &String, user: User) -> Result<(), mongodb::error::Error>;
    async fn get_tokens_and_ids(db: mongodb::Collection<bson::Document>) -> Result<Vec<(String, String)>, mongodb::error::Error>;
    async fn update_courses_info(db: mongodb::Collection<bson::Document>, token: &String, courses: Vec<Course>) -> Result<(), mongodb::error::Error>;
}