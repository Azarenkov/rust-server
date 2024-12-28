use mongodb::error::Error as mongodbErr;
use crate::adapters::{http_and_db_models::deadline::Deadline, utils::errors::DbErrors};

pub trait DeadlineRepositoryAbstract {
    async fn get_deadlines(&self, token: &String) -> Result<Option<Vec<Deadline>>, DbErrors>;
    async fn update_deadline_info(&self, token: &String, deadlines: Vec<Deadline>) -> Result<(), mongodbErr>;
}