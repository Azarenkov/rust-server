use mongodb::error::Error as MongoError;
use reqwest::Error as ReqwestError;

#[derive(Debug)]
pub enum SyncError {
    ApiError(ReqwestError),
    DatabaseError(MongoError),
    NotFound()
}

impl From<ReqwestError> for SyncError {
    fn from(err: ReqwestError) -> SyncError {
        SyncError::ApiError(err)
    }
}

impl From<MongoError> for SyncError {
    fn from(err: MongoError) -> SyncError {
        SyncError::DatabaseError(err)
    }
}