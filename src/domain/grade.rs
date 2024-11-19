use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Grades {
    pub usergrades: Vec<GradeItems>
}

#[derive(Debug, Deserialize)]
pub struct GradeItems {
    pub gradeitems: Vec<Grade>
}

#[derive(Debug, Deserialize)]
pub struct Grade {
    pub itemname: String,
    pub percentageformatted: String
}