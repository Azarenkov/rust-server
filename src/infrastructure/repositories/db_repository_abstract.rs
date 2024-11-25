use crate::domain::deadline::Deadline;
use crate::domain::full_info::UserCourseInfo;
use crate::domain::grade::GradeItems;
use crate::domain::user::User;
use crate::domain::course::Course;

pub trait DbRepositoryAbstract {
    async fn get_users_tokens(&self) -> Result<Vec<String>, mongodb::error::Error>;
    async fn update_user_info(&self, token: &String, user: User) -> Result<(), mongodb::error::Error>;
    async fn get_tokens_and_ids(&self) -> Result<Vec<(String, String)>, mongodb::error::Error>;
    async fn update_courses_info(&self, token: &String, courses: Vec<Course>) -> Result<(), mongodb::error::Error>;
    async fn get_tokens_and_userdid_and_courses(&self) -> Result<Vec<UserCourseInfo>, mongodb::error::Error>;
    async fn update_grades_info(&self, token: &String, grades: Vec<GradeItems>) -> Result<(), mongodb::error::Error>;
    async fn update_deadline_info(&self, token: &String, deadlines: Vec<Deadline>) -> Result<(), mongodb::error::Error>;
}