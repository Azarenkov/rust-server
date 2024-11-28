mod domain;
mod adapters;
mod infrastructure;
mod application;

use std::error::Error;
use actix_web::{App, HttpServer};
use application::services::actix_service::{check_token, get_deadlines, get_grades};
use infrastructure::db::get_database;
use tokio::time::{sleep, Duration};
use application::services::{actix_service::get_user_info, actix_service::get_courses, sync_service::SyncService};
use application::repositories::sync_service_abstract::SyncServiceAbstract;
use actix_web::web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let db = get_database().await;

    let service = SyncService::new(db.clone());
    service.sync_data_with_database().await;
    service.sync_courses_with_database().await;
    service.sync_grades_with_database().await;
    service.sync_deadlines_with_database().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .service(check_token)
            .service(get_user_info)
            .service(get_courses)
            .service(get_grades)
            .service(get_deadlines)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}

