use async_trait::async_trait;
use tokio::sync::mpsc;
use crate::{adapters::messaging::fcm_adapter::FcmAdapter, application::utils::errors::SyncError};

#[async_trait]
pub trait SyncCoursesWithDatabase {
    async fn sync_courses_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
}