use mongodb::bson::{self};

use crate::domain::user::User;


pub async fn update_user_info(db: mongodb::Collection<bson::Document>, token: &String, user: User) -> Result<(), mongodb::error::Error> {
    match db.update_one(
        bson::doc! {"token": token},
        bson::doc! {
            "$set": {"user_info": bson::to_bson(&user).unwrap()}
        },
        None
    ).await {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            Err(e)
        },
    }
}