use actix_web::{get, web, HttpResponse};
use mongodb::{bson::Document, Collection};
use crate::{adapters::db::db_adapter::DbAdapter, infrastructure::repositories::db_repository_abstract::DbRepositoryAbstract};
use crate::adapters::utils::errors::DbErrors;

#[get("/get_user_info/{token}")]
async fn get_user_info(token: web::Path<String>, db: web::Data<Collection<Document>>) -> HttpResponse {
    let token = token.into_inner();
    let db = DbAdapter::new(db.get_ref().clone());

    match db.get_user_info(&token).await {
        Ok(user_info) => HttpResponse::Ok().json(user_info),
        Err(e) => {
            match e {
                DbErrors::NotFound() => HttpResponse::NotFound().body(format!("No user with token: {}", token)),
                DbErrors::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}

#[get("/get_courses/{token}")]
async fn get_courses(token: web::Path<String>, db: web::Data<Collection<Document>>) -> HttpResponse {
    let token = token.into_inner();
    let db = DbAdapter::new(db.get_ref().clone());

    match db.get_courses(&token).await {
        Ok(courses) => HttpResponse::Ok().json(courses),
        Err(e) => {
            match e {
                DbErrors::NotFound() => HttpResponse::NotFound().body(format!("No courses with token: {}", token)),
                DbErrors::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}

#[get("/get_grades/{token}")]
async fn get_grades(token: web::Path<String>, db: web::Data<Collection<Document>>) -> HttpResponse {
    let token = token.into_inner();
    let db = DbAdapter::new(db.get_ref().clone());

    match db.get_grades(&token).await {
        Ok(grades) => HttpResponse::Ok().json(grades),
        Err(e) => {
            match e {
                DbErrors::NotFound() => HttpResponse::NotFound().body(format!("No grades with token: {}", token)),
                DbErrors::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}

#[get("/get_deadlines/{token}")]
async fn get_deadlines(token: web::Path<String>, db: web::Data<Collection<Document>>) -> HttpResponse {
    let token = token.into_inner();
    let db = DbAdapter::new(db.get_ref().clone());

    match db.get_deadlines(&token).await {
        Ok(deadlines) => HttpResponse::Ok().json(deadlines),
        Err(e) => {
            match e {
                DbErrors::NotFound() => HttpResponse::NotFound().body(format!("No deadlines with token: {}", token)),
                DbErrors::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}