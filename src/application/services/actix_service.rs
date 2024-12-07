use actix_web::{get, post, web, HttpResponse};
use mongodb::{bson::Document, Collection};
use crate::application::repositories::sync_service_abstract::SyncServiceAbstract;
use crate::application::services::sync_service::SyncService;
use crate::{adapters::db::db_adapter::DbAdapter, adapters::api::client::ApiClient, infrastructure::repositories::db_repository_abstract::DbRepositoryAbstract};
use crate::adapters::utils::errors::DbErrors;
use crate::domain::auth_notification_tokens::Tokens;

#[post("/add_token")]
async fn check_token(form: web::Form<Tokens>, db: web::Data<Collection<Document>>) -> HttpResponse {
    // let token = token.into_inner();

    let token = &form.token;

    let service = SyncService::new(db.get_ref().clone());
    let db = DbAdapter::new(db.get_ref().clone());
    let api_client = ApiClient::new(&token, None, None);

    match api_client.validate_token().await {
        Ok(_) => {
            match db.find_token(&token).await {
                Ok(_) => HttpResponse::Ok().body("Token is valid"),
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => {
                            match db.add_token(&token).await {
                                Ok(_) => {
                                    tokio::spawn(async move {
                                        if let Err(e) = service.sync_all_data().await {
                                            eprintln!("Error syncing data: {:?}", e);
                                        }
                                    });
                                    HttpResponse::Ok().body("Token is valid")
                                },
                                Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
                            }
                        },
                        DbErrors::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
                    }
                },
            }
        },
        Err(_) =>  HttpResponse::NotFound().body(format!("No user with token: {}", token)),
    }
}

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