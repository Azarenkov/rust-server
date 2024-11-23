use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Course {
    pub id: i64,
    pub fullname: String,
    // category: String,
    pub completed: Option<bool>,
    // start_date: i64,
    // end_date: i64,
}

