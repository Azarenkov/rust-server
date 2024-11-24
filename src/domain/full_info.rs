use serde::{Deserialize, Serialize};

use super::course::Course;


#[derive(Debug, Serialize, Deserialize)]
pub struct UserCourseInfo {
    pub token: String,
    pub user_id: i64,
    pub courses: Vec<Course>,
}