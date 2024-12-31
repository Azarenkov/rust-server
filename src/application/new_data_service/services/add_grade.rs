use crate::{adapters::{db::{interfaces::{grade_repository_abstract::GradeRepositoryAbstract, token_repository_abstract::TokenRepositoryAbstract}, model::DbAdapter}, http::http_client_repository::ApiClient}, application::{new_data_service::interfaces::add_grade_abstract::AddGradeAbstact, utils::errors::SyncError}};

impl AddGradeAbstact for DbAdapter {
    async fn add_grade(&self, token: &String) -> Result<(), SyncError> {
        let user_data = self.get_user_id_and_courses_id(&token).await?;
        let mut grades_data = Vec::new();

        for course in user_data.courses {
            let api_client = ApiClient::new(&token, Some(user_data.user_id.to_string()), Some(course.id.to_string()));
            let grades = api_client.get_grades().await?;
            grades.usergrades.clone().into_iter().for_each(|mut grade|{
                grade.coursename = Some(course.fullname.clone());
                grades_data.push(grade);
            });
        }

        self.update_grades_info(&token, grades_data).await?;
        Ok(())
    }
    
    async fn add_grade_overview(&self, token: &String) -> Result<(), SyncError> {
        let user_data = self.get_user_id_and_courses_id(&token).await?;

        let api_client = ApiClient::new(&token, None, None);
        let grades_overview = api_client.get_grades_overview().await?;
        let mut grades = grades_overview.grades;

        for course in user_data.courses.iter() {
            for grade in grades.iter_mut() {
                if grade.courseid == course.id {
                    grade.course_name = Some(course.fullname.clone())
                }
            }
        }

        self.update_grades_overview(&token, &grades).await?;
        Ok(())
    }
}