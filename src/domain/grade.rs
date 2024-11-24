use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Grades {
    pub usergrades: Vec<GradeItems>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GradeItems {
    pub coursename: Option<String>,
    pub courseid: i64,
    pub gradeitems: Vec<Grade>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Grade {
    pub itemname: String,
    pub percentageformatted: String
}