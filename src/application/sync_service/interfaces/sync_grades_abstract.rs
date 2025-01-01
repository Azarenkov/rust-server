use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::{adapters::messaging::fcm_adapter::FcmAdapter, application::utils::errors::SyncError};

#[async_trait]
pub trait SyncGradesWithDatabase {
    async fn sync_grades_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
    async fn sync_grades_overview_with_databse(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
}