use actix_web::{test, web, App};
use rust_server::adapters::{api::actix_controller::get_user_info, db::db_connection::get_database, http_and_db_models::user::User};
use serde_json::Value;

use crate::utils::{db_utils::DbAdapterTest, file_utils::read_from_file};

#[actix_web::test]
async fn get_user_info_by_token() {
    let db = get_database().await;
    let db_test = DbAdapterTest::new(&db).await;
    let token_test = String::from("1");

    // insert data in db
    db_test.execute_imports::<User>(&token_test).await;

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db))
            .service(get_user_info)
    ).await;

    let req = test::TestRequest::get()
        .uri(&format!("/get_user_info/{}", token_test))
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());

    let resp_json: Value = test::read_body_json(resp).await;
    let expected_json: Value = read_from_file("tests/fixtures/user_info.json").unwrap();

    assert_eq!(resp_json, expected_json);
    
    // delete data from db
    db_test.delete_imports(&token_test).await;
}