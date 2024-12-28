use tokio::sync::mpsc;

use crate::{adapters::messaging::fcm_adapter::FcmAdapter, application::utils::errors::SyncError};

pub trait SyncDeadlinesWithDatabase {
    async fn sync_deadlines_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
}