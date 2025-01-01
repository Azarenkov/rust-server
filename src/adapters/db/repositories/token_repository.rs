use async_trait::async_trait;
use futures_util::TryStreamExt;
use mongodb::{bson::{self, doc}, error::Error as mongodbErr};

use crate::adapters::{db::{interfaces::token_repository_abstract::TokenRepositoryAbstract, model::DbAdapter}, http_and_db_models::{course::Course, db_user_course_info::UserCourseInfo}, utils::errors::DbErrors};

#[async_trait]
impl TokenRepositoryAbstract for DbAdapter {
    
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
                                token: Some(token.to_string()),
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
    
    async fn delete_document(&self, token: &String) -> Result<(), mongodbErr> {
        self.collection.delete_one(doc! { "token": token }, None).await?;
        println!("Document deleted!");

        Ok(())
    }
    
    async fn get_user_id(&self, token: &String) -> Result<String, mongodbErr> {
        let filter = doc! {
            "token": token,
            "user_info": { "$exists": true }
        };
    
        let mut cursor = self.collection.find(filter, None).await?;
        let mut user_id = String::new();
        while let Some(doc) = cursor.try_next().await? {
            if let Ok(user_info_doc) = doc.get_document("user_info") {
                if let Ok(uid) = user_info_doc.get_i64("userid") {
                    user_id = uid.to_string();
                }
            }
        }
        Ok(user_id)
    }
    
    async fn get_user_id_and_courses_id(&self, token: &String) -> Result<UserCourseInfo, mongodbErr> {
        let mut user_id_and_courses = UserCourseInfo {
            token: None,
            user_id: 0,
            courses: Vec::new(),
        };
        let filter = doc! {"token": token, "user_info": {"$exists": true}, "courses": {"$exists": true}};
        let mut cursor = self.collection.find(filter, None).await?;
        while let Some(doc) = cursor.try_next().await? {
                if let Some(user_info) = doc.get_document("user_info").ok() {
                    if let Some(user_id) = user_info.get_i64("userid").ok() {
                        if let Some(courses_info) = doc.get_array("courses").ok() {
                            let courses: Vec<Course> = courses_info.iter().filter_map(|course| bson::from_bson(course.clone()).ok()).collect();
                            user_id_and_courses = UserCourseInfo {
                                token: None,
                                user_id,
                                courses,
                            };
                        }
                    }
                }
            
        }
        Ok(user_id_and_courses)
    }
    
    
    
}