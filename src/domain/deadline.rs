use serde::{Deserialize, Serialize};

use super::course::Course;

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Events {
    pub events: Vec<Deadline>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Deadline {
    pub name: String,
    pub timeusermidnight: i64,
    pub course: Course,
    pub formattedtime: String,
}