use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Events {
    pub events: Vec<Deadline>,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
pub struct Deadline {
    pub name: String,
    pub timeusermidnight: i64,
    pub formattedtime: String,
    pub coursename: Option<String>
}