use std::fmt::format;

use reqwest::Client;
use super::helpers::Functions;
use crate::domain::user::User;

// use crate::user_traits::UserTrait;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: String,
    format: String,
}

impl ApiClient {
    pub fn new(token: &str) -> Self {
        ApiClient {
            client: Client::new(),
            base_url: "https://moodle.astanait.edu.kz/webservice/rest/server.php?".to_string(),
            token: format!("wstoken={}", token),
            format: "&moodlewsrestformat=json".to_string(),
        }
    }

    pub async fn get_user(&self) -> Result<User, reqwest::Error> {
        let function = Functions::GetUserData.new();

        let url = format!("{}{}{}{}",
            self.base_url,
            self.token,
            function,
            self.format
        );

        let response = self.client.get(&url).send().await?.json::<User>().await?;
        Ok(response)
    }
}