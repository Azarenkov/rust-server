use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Ord, Eq, PartialOrd)]
pub struct Grades {
    pub usergrades: Vec<GradeItems>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Ord, Eq, PartialOrd)]
pub struct GradeItems {
    pub coursename: Option<String>,
    pub courseid: i64,
    pub gradeitems: Vec<Grade>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default, Ord, Eq, PartialOrd)]
pub struct Grade {
    pub itemname: String,
    pub percentageformatted: String
}