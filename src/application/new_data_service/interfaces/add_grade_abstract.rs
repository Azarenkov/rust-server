use async_trait::async_trait;

use crate::application::utils::errors::SyncError;

#[async_trait]
pub trait AddGradeAbstact {
    async fn add_grade(&self, token: &String) -> Result<(), SyncError>;
    async fn add_grade_overview(&self, token: &String) -> Result<(), SyncError>;
}