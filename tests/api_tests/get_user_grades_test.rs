use actix_web::{test, web, App};
use rust_server::adapters::{api::actix_controller::get_grades, db::{db_connection::get_database, model::DbAdapter}, http_and_db_models::grade::GradeItems};

use crate::fixtures::fixtures_run::DbAdabterTestAbstract;


#[actix_web::test]
async fn get_user_grades_by_token() {
    let db = get_database().await;
    let db_test = DbAdapter::new(db.clone());
    let token_test = String::from("3");

    // insert data in db
    db_test.execute_imports::<Vec<GradeItems>>(&token_test).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .service(get_grades)
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/get_grades/{}", token_test))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
    
    // delete data from db
    db_test.delete_imports(&token_test).await;
}