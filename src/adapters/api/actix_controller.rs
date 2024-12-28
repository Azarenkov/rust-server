use actix_web::{delete, get, post, web, HttpResponse};
use mongodb::{bson::Document, Collection};
use tokio::task;
use crate::adapters::api::info_payloads::Tokens;
use crate::adapters::db::interfaces::course_repository_abstract::CourseRepositoryAbstract;
use crate::adapters::db::interfaces::deadline_repository_abstract::DeadlineRepositoryAbstract;
use crate::adapters::db::interfaces::grade_repository_abstract::GradeRepositoryAbstract;
use crate::adapters::db::interfaces::token_repository_abstract::TokenRepositoryAbstract;
use crate::adapters::db::interfaces::user_repository_abstract::UserRepositoryAbstract;
use crate::adapters::db::model::DbAdapter;
use crate::adapters::http::http_client_repository::ApiClient;
use crate::application::sync_service::interfaces::sync_service_abstract::SyncServiceAbstract;
use crate::application::sync_service::sync_service::SyncService;
use crate::adapters::utils::errors::DbErrors;

#[post("/add_token")]
async fn check_token(form: web::Json<Tokens>, db: web::Data<Collection<Document>>) -> HttpResponse {
    // let token = token.into_inner();

    let token = &form.token;

    let service = SyncService::new(db.get_ref().clone());

    let db = DbAdapter::new(db.get_ref().clone());
    let api_client = ApiClient::new(&token, None, None);

    match api_client.validate_token().await {
        Ok(_) => {
            match db.find_token(&token).await {
                Ok(_) => {
                    if let Some(device_token) = &form.device_token {
                        match db.add_device_token(token, device_token).await {
                            Ok(_) => {
                                HttpResponse::Ok().body("Token is valid")
                            },
                            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
                        }                                  
                    } else {
                        HttpResponse::Ok().body("Token is valid")
                    }
                },
                
                Err(e) => {
                    match e {
                        DbErrors::NotFound() => {
                            match db.add_token(&token).await {
                                Ok(_) => {
                                    if let Some(device_token) = &form.device_token {
                                        match db.add_device_token(token, device_token).await {
                                            Ok(_) => {
                                                task::spawn(async move {
                                                    if let Err(e) = service.sync_all_data(None).await {
                                                        eprintln!("Error syncing data: {:?}", e);
                                                    }
                                                });
                                                HttpResponse::Ok().body("Token is valid")
                                            },
                                            Err(e) => HttpResponse::InternalServerError().body(e.to_string())
                                        }                                  
                                    } else {

                                    tokio::spawn(async move {
                                        if let Err(e) = service.sync_all_data(None).await {
                                            eprintln!("Error syncing data: {:?}", e);
                                        }
                                    });
                                    HttpResponse::Ok().body("Token is valid")
                                }
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

#[get("/get_grades_overview/{token}")]
async fn get_grades_overview(token: web::Path<String>, db: web::Data<Collection<Document>>) -> HttpResponse {
    let token = token.into_inner();
    let db = DbAdapter::new(db.get_ref().clone());

    match db.get_grades_overview(&token).await {
        Ok(grades_overview) => HttpResponse::Ok().json(grades_overview),
        Err(e) => {
            match e {
                DbErrors::NotFound() => HttpResponse::NotFound().body(format!("No grades_overview with token: {}", token)),
                DbErrors::DbError(e) => HttpResponse::InternalServerError().body(e.to_string()),
            }
        }
    }
}

#[delete("/delete_document/{token}")]
async fn delete_docment(token: web::Path<String>, db: web::Data<Collection<Document>>) -> HttpResponse {
    let token = token.into_inner();
    let db = DbAdapter::new(db.get_ref().clone());

    match db.delete_document(&token).await {
        Ok(_) => HttpResponse::Ok().body("Document removed"),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
    
}