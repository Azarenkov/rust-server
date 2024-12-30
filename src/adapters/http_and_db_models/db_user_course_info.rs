use serde::{Deserialize, Serialize};

use super::course::Course;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCourseInfo {
    pub token: Option<String>,
    pub user_id: i64,
    pub courses: Vec<Course>,
}