use actix_web::{web, App, HttpServer};
use mongodb::{bson::Document, Collection};
use actix_web::Error;
use tokio::sync::mpsc;

use crate::adapters::api::actix_controller::{check_token, delete_docment, get_courses, get_deadlines, get_grades, get_grades_overview, get_user_info};

pub async fn get_web_server(db: Collection<Document>) -> Result<(), Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(check_token)
            .service(get_user_info)
            .service(get_courses)
            .service(get_grades)
            .service(get_deadlines)
            .service(get_grades_overview)
            .service(delete_docment)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}