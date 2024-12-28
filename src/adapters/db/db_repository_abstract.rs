use mongodb::error::Error as mongodbErr;
use crate::adapters::http_and_db_models::course::Course;
use crate::adapters::http_and_db_models::db_user_course_info::UserCourseInfo;
use crate::adapters::http_and_db_models::deadline::Deadline;
use crate::adapters::http_and_db_models::grade::GradeItems;
use crate::adapters::http_and_db_models::grade_overview::GradeOverview;
use crate::adapters::http_and_db_models::user::User;
use crate::adapters::utils::errors::DbErrors;


pub trait DbRepositoryAbstract {
    async fn get_users_tokens(&self) -> Result<Vec<String>, mongodbErr>;
    async fn get_tokens_and_ids(&self) -> Result<Vec<(String, String)>, mongodbErr>;
    async fn get_tokens_and_userdid_and_courses(&self) -> Result<Vec<UserCourseInfo>, mongodbErr>;
    async fn get_user_info(&self, token: &String) -> Result<User, DbErrors>;
    async fn get_courses(&self, token: &String) -> Result<Vec<Course>, DbErrors>;
    async fn get_grades(&self, token: &String) -> Result<Vec<GradeItems>, DbErrors>;
    async fn get_deadlines(&self, token: &String) -> Result<Option<Vec<Deadline>>, DbErrors>;
    async fn get_device_token(&self, token: &String) -> Result<String, DbErrors>;
    async fn get_grades_overview(&self, token: &String) -> Result<Vec<GradeOverview>, DbErrors>;

    async fn update_user_info(&self, token: &String, user: User) -> Result<(), mongodbErr>;
    async fn update_courses_info(&self, token: &String, courses: Vec<Course>) -> Result<(), mongodbErr>;
    async fn update_grades_info(&self, token: &String, grades: Vec<GradeItems>) -> Result<(), mongodbErr>;
    async fn update_deadline_info(&self, token: &String, deadlines: Vec<Deadline>) -> Result<(), mongodbErr>;
    async fn update_grades_overview(&self, token: &String, grades_overview: &Vec<GradeOverview>) -> Result<(), mongodbErr>;

    async fn add_token(&self, token: &String) -> Result<(), mongodbErr>;
    async fn find_token(&self, token: &String) -> Result<(), DbErrors>;
    async fn add_device_token(&self, token: &String, device_token: &String) -> Result<(), mongodbErr>;

    async fn delete_document(&self, token: &String) -> Result<(), mongodbErr>;
}