use tokio::sync::mpsc;

use crate::{adapters::messaging::fcm_adapter::FcmAdapter, application::utils::errors::SyncError};

pub trait SyncServiceAbstract {
    async fn sync_all_data(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
}
