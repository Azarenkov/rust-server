use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GradesOverview {
    grades: Vec<GradeOverview>
}

#[derive(Debug, Deserialize)]
pub struct GradeOverview {
    courseid: i32,
    grade: String,
    rawgrade: String,
}