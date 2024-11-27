use futures_util::TryStreamExt;
use mongodb::bson::{self, doc, Array, Document};
use mongodb::Collection;
use mongodb::error::Error as mongodbErr;
use crate::domain::course::Course;
use crate::domain::deadline::Deadline;
use crate::domain::full_info::UserCourseInfo;
use crate::domain::grade::GradeItems;
use crate::domain::user::User;
use crate::infrastructure::repositories::db_repository_abstract::DbRepositoryAbstract;
use crate::adapters::utils::errors::DbErrors;


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
        match self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"user_info": bson::to_bson(&user).unwrap()}
            },
            None
        ).await {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e),
        }
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
        match self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"courses": bson::to_bson(&courses).unwrap()}
            },
            None
        ).await {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e),
        }
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
        match self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"grades": bson::to_bson(&grades).unwrap()}
            },
            None
        ).await {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e),
        }
    }
    
    async fn update_deadline_info(&self, token: &String, deadlines: Vec<Deadline>) -> Result<(), mongodbErr> {
        match self.collection.update_one(
            bson::doc! {"token": token},
            bson::doc! {
                "$set": {"deadlines": bson::to_bson(&deadlines).unwrap()}
            },
            None
        ).await {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e),
        }
    }
    

    async fn get_user_info(&self, token: &String) -> Result<Document, DbErrors> {
        match self.collection.find_one(doc! { "token": &token }, None).await {
            Ok(document) => {
                match document {
                    Some(doc) => {
                        if let Some(user_info) = doc.get_document("user_info").ok() {
                            Ok(user_info.clone())
                        } else {
                            Err(DbErrors::NotFound())
                        }
                    },
                    None => Err(DbErrors::NotFound()),
                }
            },
            Err(e) => Err(DbErrors::DbError(e))
        }
    }
    
    async fn get_courses(&self, token: &String) -> Result<Array, DbErrors> {
        match self.collection.find_one(doc! { "token": &token }, None).await {
            Ok(document) => {
                match document {
                    Some(doc) => {
                        if let Some(courses) = doc.get_array("courses").ok() {
                            Ok(courses.clone())
                        } else {
                            Err(DbErrors::NotFound())
                        }
                    },
                    None => Err(DbErrors::NotFound()),
                }
            },
            Err(e) => Err(DbErrors::DbError(e))
        }
    }
    
    async fn get_grades(&self, token: &String) -> Result<Array, DbErrors> {
        match self.collection.find_one(doc! { "token": &token }, None).await {
            Ok(document) => {
                match document {
                    Some(doc) => {
                        if let Some(grades) = doc.get_array("grades").ok() {
                            Ok(grades.clone())
                        } else {
                            Err(DbErrors::NotFound())
                        }
                    },
                    None => Err(DbErrors::NotFound()),
                }
            },
            Err(e) => Err(DbErrors::DbError(e))
        }
    }
    
    async fn get_deadlines(&self, token: &String) -> Result<Array, DbErrors> {
        match self.collection.find_one(doc! { "token": &token }, None).await {
            Ok(document) => {
                match document {
                    Some(doc) => {
                        if let Some(deadlines) = doc.get_array("deadlines").ok() {
                            Ok(deadlines.clone())
                        } else {
                            Err(DbErrors::NotFound())
                        }
                    },
                    None => Err(DbErrors::NotFound()),
                }
            },
            Err(e) => Err(DbErrors::DbError(e))
        }
    }

    
    
}
