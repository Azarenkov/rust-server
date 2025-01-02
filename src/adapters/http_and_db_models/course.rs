use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Ord, PartialOrd, PartialEq, Eq, Debug)]
pub struct Course {
    pub id: i64,
    pub fullname: String,
    // category: String,
    pub completed: Option<bool>,
    pub enddate: i64,
}

impl ToString for Course {
    fn to_string(&self) -> String {
        format!("Course {{ id: {}, fullname: {}, completed: {:?} }}", self.id, self.fullname, self.completed)
    }
}