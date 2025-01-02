#[cfg(test)]
mod tests {
    use crate::adapters::{http::http_client_repository::ApiClient, usecases::mock_api_client::ApiClientMockAbstract};
    use mockito::Server;
    use tokio;

    #[tokio::test]
    async fn test_get_courses() {

        // Creating mock server
        let mut server = Server::new_async().await;
        let url = server.url();
        let token = String::from("711abc349948337f8b97cbb01b76adf5");
        let user_id = String::from("19401");

        // Configuration of the server 
        let mock = server.mock("GET", "/webservice/rest/server.php")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("wstoken".into(), token.clone().into()),
                mockito::Matcher::UrlEncoded("wsfunction".into(), "core_enrol_get_users_courses".into()),
                mockito::Matcher::UrlEncoded("moodlewsrestformat".into(), "json".into()),
                mockito::Matcher::UrlEncoded("userid".into(), user_id.clone().into()),
            ]))
            // .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"[{"id": 5286, "fullname": "fullname 1", "enddate": 1733011200}, {"id": 1231, "fullname": "fullname 2", "enddate": 1733011240}]"#)
            .create_async().await;
        
        // Request
        let client = ApiClient::new_with_base_url(&token, Some(user_id), None, &url);
        let result = client.get_courses().await;

        assert!(result.is_ok());
        let courses = result.unwrap();
        
        assert_eq!(courses.len(), 2);

        assert_eq!(courses[0].id, 5286);
        assert_eq!(courses[0].fullname, "fullname 1");
        assert_eq!(courses[0].enddate, 1733011200);
        assert_eq!(courses[1].id, 1231);
        assert_eq!(courses[1].fullname, "fullname 2");
        assert_eq!(courses[1].enddate, 1733011240);
        assert!(courses.get(2).is_none());

        mock.assert();
    }
}