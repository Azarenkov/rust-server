use async_trait::async_trait;
use mongodb::error::Error as mongodbErr;
use crate::adapters::{http_and_db_models::{grade::GradeItems, grade_overview::GradeOverview}, utils::errors::DbErrors};

#[async_trait]
pub trait GradeRepositoryAbstract {
    async fn get_grades(&self, token: &String) -> Result<Vec<GradeItems>, DbErrors>;
    async fn update_grades_info(&self, token: &String, grades: Vec<GradeItems>) -> Result<(), mongodbErr>;
    async fn get_grades_overview(&self, token: &String) -> Result<Vec<GradeOverview>, DbErrors>;
    async fn update_grades_overview(&self, token: &String, grades_overview: &Vec<GradeOverview>) -> Result<(), mongodbErr>;
}