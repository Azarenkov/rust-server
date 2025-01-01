use async_trait::async_trait;
use mongodb::{bson::{self, doc}, error::Error as mongodbErr};

use crate::adapters::{db::{interfaces::course_repository_abstract::CourseRepositoryAbstract, model::DbAdapter}, http_and_db_models::course::Course, utils::errors::DbErrors};

#[async_trait]
impl CourseRepositoryAbstract for DbAdapter {
    async fn get_courses(&self, token: &String) -> Result<Vec<Course>, DbErrors> {
        let document = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| {
            DbErrors::DbError(e)
        })?;

        match document {
            Some(doc) => {
                if let Some(courses) = doc.get_array("courses").ok() {
                    let courses: Vec<Course> = courses.iter().filter_map(|course| bson::from_bson(course.clone()).ok()).collect();
                    Ok(courses)
                } else {
                    Err(DbErrors::NotFound())
                }
            },
            None => Err(DbErrors::NotFound()),
        }
    }

    async fn update_courses_info(&self, token: &String, courses: Vec<Course>) -> Result<(), mongodbErr> {
        self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"courses": bson::to_bson(&courses).unwrap()}
            },
            None
        ).await?;
        println!("Courses updated!");

        Ok(())
    }
}