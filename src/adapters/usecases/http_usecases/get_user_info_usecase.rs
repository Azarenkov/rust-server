#[cfg(test)]
mod tests {
    use crate::adapters::{http::http_client_repository::ApiClient, usecases::mock_api_client::ApiClientMockAbstract};
    use mockito::Server;
    use tokio;

    #[tokio::test]
    async fn test_get_user() {

        // Creating mock server
        let mut server = Server::new_async().await;
        let url = server.url();
        let token = String::from("711abc349948337f8b97cbb01b76adf5");

        // Configuration of the server 
        let mock = server.mock("GET", "/webservice/rest/server.php")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("wstoken".into(), token.clone().into()),
                mockito::Matcher::UrlEncoded("wsfunction".into(), "core_webservice_get_site_info".into()),
                mockito::Matcher::UrlEncoded("moodlewsrestformat".into(), "json".into())
            ]))
            // .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"userid": 1, "fullname": "Test fullname", "username": "Test username"}"#)
            .create_async().await;
        
        // Request
        let client = ApiClient::new_with_base_url(&token, None, None, &url);
        let result = client.get_user().await;

        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.userid, Some(1));
        assert_eq!(user.fullname, Some(String::from("Test fullname")));
        assert_eq!(user.username, Some(String::from("Test username")));

        mock.assert();
    }
}

