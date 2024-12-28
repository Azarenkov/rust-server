use mongodb::{bson::{self, doc}, error::Error as mongodbErr};

use crate::adapters::{db::{interfaces::grade_repository_abstract::GradeRepositoryAbstract, model::DbAdapter}, http_and_db_models::{grade::GradeItems, grade_overview::GradeOverview}, utils::errors::DbErrors};

impl GradeRepositoryAbstract for DbAdapter {
    async fn get_grades(&self, token: &String) -> Result<Vec<GradeItems>, DbErrors> {
        let document = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| {
            DbErrors::DbError(e)
        })?;

        match document {
            Some(doc) => {
                if let Some(grades) = doc.get_array("grades").ok() {
                    let grades = grades.iter().filter_map(|grade| bson::from_bson(grade.clone()).ok()).collect();
                    Ok(grades)
                } else {
                    Err(DbErrors::NotFound())
                }
            },
            None => Err(DbErrors::NotFound()),
        }
    }

        
    async fn update_grades_info(&self, token: &String, grades: Vec<GradeItems>) -> Result<(), mongodbErr> {
        self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"grades": bson::to_bson(&grades).unwrap()}
            },
            None
        ).await?;
        println!("Grades updated!");

        Ok(())
    }

    async fn get_grades_overview(&self, token: &String) -> Result<Vec<GradeOverview>, DbErrors> {
        let document = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| {
            DbErrors::DbError(e)
        })?;

        match document {
            Some(doc) => {
                if let Some(grades_overview) = doc.get_array("grades_overview").ok() {
                    let grades_overview: Vec<GradeOverview> = grades_overview.iter().filter_map(|grade| bson::from_bson(grade.clone()).ok()).collect();
                    Ok(grades_overview)
                } else {
                    Err(DbErrors::NotFound())
                }
            },
            None => Err(DbErrors::NotFound()),
        }
    }
    
    async fn update_grades_overview(&self, token: &String, grades_overview: &Vec<GradeOverview>) -> Result<(), mongodbErr> {
        self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"grades_overview": bson::to_bson(&grades_overview).unwrap()}
            },
            None
        ).await?;
        println!("Grades_overview updated!");

        Ok(())
    }
}