use serde::Deserialize;

use super::course::Course;

#[derive(Debug, Deserialize)]
pub struct Events {
    pub events: Vec<Deadline>,
}

#[derive(Debug, Deserialize)]
pub struct Deadline {
    pub name: String,
    pub timeusermidnight: i64,
    pub course: Course,
    pub formattedtime: String,
}