use async_trait::async_trait;
use tokio::sync::mpsc;
use crate::{adapters::messaging::fcm_adapter::FcmAdapter, application::utils::errors::SyncError};

#[async_trait]
pub trait SyncDataWithDatabase {
    async fn sync_data_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
}