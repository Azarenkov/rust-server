use tokio::sync::mpsc;

use crate::{adapters::messaging::fcm_adapter::FcmAdapter, application::utils::errors::SyncError};

pub trait SyncServiceAbstract {
    async fn sync_data_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
    async fn sync_courses_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
    async fn sync_grades_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
    async fn sync_deadlines_with_database(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
    async fn sync_grades_overview_with_databse(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;
    async fn sync_all_data(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError>;

}
