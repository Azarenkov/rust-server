use mongodb::{bson::{self, doc}, error::Error as mongodbErr};

use crate::adapters::{db::{interfaces::deadline_repository_abstract::DeadlineRepositoryAbstract, model::DbAdapter}, http_and_db_models::deadline::Deadline, utils::errors::DbErrors};

impl DeadlineRepositoryAbstract for DbAdapter {
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
}