use mongodb::error::Error as MongoError;
use reqwest::Error as ReqwestError;
use serde_json::error::Error as JsonError;
use chrono::ParseError;


#[derive(Debug)]
pub enum SyncError {
    ApiError(ReqwestError),
    DatabaseError(MongoError),
    SerdeError(JsonError),
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