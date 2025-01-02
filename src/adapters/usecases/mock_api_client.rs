use crate::adapters::http::http_client_repository::ApiClient;
use reqwest::Client;

pub trait ApiClientMockAbstract {
    fn new_with_base_url(token: &str, user_id: Option<String>, course_id: Option<String>, base_url: &str) -> Self;
}

impl ApiClientMockAbstract for ApiClient {
     fn new_with_base_url(token: &str, user_id: Option<String>, course_id: Option<String>, base_url: &str) -> Self {
        ApiClient {
            client: Client::new(),
            base_url: format!("{}/webservice/rest/server.php?", base_url),
            token: format!("wstoken={}", token),
            format: "&moodlewsrestformat=json".to_string(),
            user_id: user_id.map(|id| format!("&userid={}", id)),
            course_id: course_id.map(|id| format!("&courseid={}", id))
        }
    }
}