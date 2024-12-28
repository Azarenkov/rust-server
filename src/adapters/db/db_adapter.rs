use futures_util::TryStreamExt;
use mongodb::bson::{self, doc, Document, Bson};
use mongodb::Collection;
use mongodb::error::Error as mongodbErr;
use crate::adapters::http_and_db_models::course::Course;
use crate::adapters::http_and_db_models::db_user_course_info::UserCourseInfo;
use crate::adapters::http_and_db_models::deadline::Deadline;
use crate::adapters::http_and_db_models::grade::GradeItems;
use crate::adapters::http_and_db_models::grade_overview::GradeOverview;
use crate::adapters::http_and_db_models::user::User;
use crate::adapters::utils::errors::DbErrors;

use super::db_repository_abstract::DbRepositoryAbstract;


pub struct DbAdapter {
    pub collection: Collection<Document>,
}

impl DbAdapter {
    pub fn new(collection: Collection<Document>) -> Self {
        DbAdapter { collection }
    }
}

impl DbRepositoryAbstract for DbAdapter {
    async fn update_user_info(&self, token: &String, user: User) -> Result<(), mongodbErr> {
        self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"user_info": bson::to_bson(&user).unwrap()}
            },
            None
        ).await?;
        println!("User_info updated!");

        Ok(())
    }
    
    async fn get_users_tokens(&self) -> Result<Vec<String>, mongodbErr> {
        let mut tokens: Vec<String> = Vec::new();
        let filter = doc! {"token": {"$exists": true}};
        let mut cursor = self.collection.find(filter, None).await?;
        while let Some(doc) = cursor.try_next().await? {
            if let Some(token) = doc.get_str("token").ok() {
                tokens.push(token.to_string());
            }
        }
        Ok(tokens)
    }
    
    async fn get_tokens_and_ids(&self) -> Result<Vec<(String, String)>, mongodbErr> {
        let mut tokens_and_ids: Vec<(String, String)> = Vec::new();
        let filter = doc! {"token": {"$exists": true}, "user_info": {"$exists": true}};
        let mut cursor = self.collection.find(filter, None).await?;
        while let Some(doc) = cursor.try_next().await? {
            if let Some(token) = doc.get_str("token").ok() {
                if let Some(user_info) = doc.get_document("user_info").ok() {
                    if let Some(id) = user_info.get_i64("userid").ok() {
                        tokens_and_ids.push((token.to_string(), id.to_string()));
                    }
                }
            }
        }
        Ok(tokens_and_ids)
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
    
    async fn get_tokens_and_userdid_and_courses(&self) -> Result<Vec<UserCourseInfo>, mongodbErr> {
        let mut tokens_and_info: Vec<UserCourseInfo> = Vec::new();
        let filter = doc! {"token": {"$exists": true}, "user_info": {"$exists": true}, "courses": {"$exists": true}};
        let mut cursor = self.collection.find(filter, None).await?;
        while let Some(doc) = cursor.try_next().await? {
            if let Some(token) = doc.get_str("token").ok() {
                if let Some(user_info) = doc.get_document("user_info").ok() {
                    if let Some(user_id) = user_info.get_i64("userid").ok() {
                        if let Some(courses_info) = doc.get_array("courses").ok() {
                            let courses: Vec<Course> = courses_info.iter().filter_map(|course| bson::from_bson(course.clone()).ok()).collect();
                            tokens_and_info.push(UserCourseInfo {
                                token: token.to_string(),
                                user_id: user_id,
                                courses,
                            });
                        }
                    }
                }
            }
        }
        Ok(tokens_and_info)
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
    
    async fn update_deadline_info(&self, token: &String, deadlines: Vec<Deadline>) -> Result<(), mongodbErr> {
        self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"deadlines": bson::to_bson(&deadlines).unwrap()}
            },
            None
        ).await?;
        println!("Deadlines updated!");

        Ok(())
    }
    

    async fn get_user_info(&self, token: &String) -> Result<User, DbErrors> {
        let document = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| {
            DbErrors::DbError(e)
        })?;

        match document {
            Some(doc) => {
                if let Some(user_info) = doc.get_document("user_info").ok() {
                    match bson::from_bson::<User>(Bson::Document(user_info.clone())) {
                        Ok(user) => Ok(user),
                        Err(e) => {
                            eprintln!("Deserialization error: {:?}", e);
                            Err(DbErrors::DbError(e.into()))
                        }
                    }
                } else {
                    Err(DbErrors::NotFound())
                }
            },
            None => Err(DbErrors::NotFound()),
        }
    }
    
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
    
    async fn get_deadlines(&self, token: &String) -> Result<Option<Vec<Deadline>>, DbErrors> {
        let document = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| {
            DbErrors::DbError(e)
        })?;

        match document {
            Some(doc) => {
                if let Some(deadlines) = doc.get_array("deadlines").ok() {
                    
                    if deadlines.is_empty() {
                        return Ok(None);
                    }
                    let deadlines: Vec<Deadline> = deadlines.iter().filter_map(|deadline| bson::from_bson(deadline.clone()).ok()).collect();

                    Ok(Some(deadlines))
                } else {
                    Err(DbErrors::NotFound())
                }
            },
            None => Err(DbErrors::NotFound()),
        }
    }
    
    async fn add_token(&self, token: &String) -> Result<(), mongodbErr> {
        self.collection.insert_one(doc! { "token": &token }, None).await?;
        Ok(())
    }
    
    async fn find_token(&self, token: &String) -> Result<(), DbErrors> {
        let doc = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| DbErrors::DbError(e))?;

        if let Some(_token) = doc {
            Ok(())
        } else {
            Err(DbErrors::NotFound())
        }
    }
    
    async fn add_device_token(&self, token: &String, device_token: &String) -> Result<(), mongodbErr> {
        self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"device_token": &device_token}
            },
            None
        ).await?;
        println!("Device token updated!");

        Ok(())
    }

    async fn get_device_token(&self, token: &String) -> Result<String, DbErrors> {
        let document = self.collection.find_one(doc! { "token": &token }, None).await.map_err(|e| {
            DbErrors::DbError(e)
        })?;

        match document {
            Some(doc) => {
                if let Some(device_token) = doc.get_str("device_token").ok() {
                    Ok(device_token.to_string())
                } else {
                    Err(DbErrors::NotFound())
                }
            },
            None => Err(DbErrors::NotFound()),
        }
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
    
    async fn delete_document(&self, token: &String) -> Result<(), mongodbErr> {
        self.collection.delete_one(doc! { "token": token }, None).await?;
        println!("Document deleted!");

        Ok(())
    }
    
}
