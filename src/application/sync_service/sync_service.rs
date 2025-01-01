use async_trait::async_trait;
use mongodb::bson::{self};
use tokio::sync::mpsc;
use crate::adapters::messaging::fcm_adapter::FcmAdapter;
use crate::application::utils::errors::SyncError;
use super::interfaces::sync_courses_abstract::SyncCoursesWithDatabase;
use super::interfaces::sync_data_abstract::SyncDataWithDatabase;
use super::interfaces::sync_deadlines_abstract::SyncDeadlinesWithDatabase;
use super::interfaces::sync_grades_abstract::SyncGradesWithDatabase;
use super::interfaces::sync_service_abstract::SyncServiceAbstract;

pub struct SyncService {
    pub db: mongodb::Collection<bson::Document>,
}

impl SyncService {
    pub fn new(db: mongodb::Collection<bson::Document>) -> Self {
        SyncService { db }
    }
}

#[async_trait]
impl SyncServiceAbstract for SyncService {

    async fn sync_all_data(&self, tx: Option<mpsc::Sender<FcmAdapter>>) -> Result<(), SyncError> {

        self.sync_data_with_database(tx.clone()).await?;
        self.sync_courses_with_database(tx.clone()).await?;
        self.sync_grades_with_database(tx.clone()).await?;
        self.sync_deadlines_with_database(tx.clone()).await?;
        self.sync_grades_overview_with_databse(tx.clone()).await?;

        Ok(())
    }
}