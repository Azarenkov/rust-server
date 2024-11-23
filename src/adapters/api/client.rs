// use std::fmt::format;

use reqwest::Client;
use super::helpers::Functions;
use crate::domain::{user, course};
use user::User;
use course::Course;

// use crate::user_traits::UserTrait;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: String,
    format: String,
    user_id: Option<String>,
    course_id: Option<String>,
}

impl ApiClient {
    pub fn new(token: &str, user_id: Option<String>, course_id: Option<String>) -> Self {
        ApiClient {
            client: Client::new(),
            base_url: "https://moodle.astanait.edu.kz/webservice/rest/server.php?".to_string(),
            token: format!("wstoken={}", token),
            format: "&moodlewsrestformat=json".to_string(),
            user_id,
            course_id
        }
    }

    pub async fn get_user(&self) -> Result<User, reqwest::Error> {
        let function = Functions::GetUserData.new();

        let url = format!("{}{}{}{}",
            self.base_url,
            self.token,
            format!("&wsfunction={}", function),
            self.format
        );
        println!("{}", url);

        let response = self.client.get(&url).send().await?.json::<User>().await?;
        Ok(response)
    }

    pub async fn get_courses(&self) -> Result<Vec<Course>, reqwest::Error> {
        let function = Functions::GetAllCourses.new();

        let url = format!("{}{}{}{}{}",
            self.base_url,
            self.token,
            format!("&wsfunction={}", function),
            self.format,
            format!("&userid={}", self.user_id.clone().unwrap_or_default())
        );
        println!("{}", url);

        let response = self.client.get(&url).send().await?.json::<Vec<Course>>().await?;
        Ok(response)
    }
}