use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct User {
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub userid: Option<i64>,
}