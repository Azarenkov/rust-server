use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GradesOverview {
    pub grades: Vec<GradeOverview>
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct GradeOverview {
    pub courseid: i64,
    pub grade: Option<String>,
    pub rawgrade: Option<String>,
    pub course_name: Option<String>,

}