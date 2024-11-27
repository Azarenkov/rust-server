use reqwest::Client;
use reqwest::Error as ReqwestErr;
use super::helpers::Functions;
use crate::domain::{course::Course, deadline::Events, grade::Grades, user::User};

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
            user_id: user_id.map(|id| format!("&userid={}", id)),
            course_id: course_id.map(|id| format!("&courseid={}", id))
        }
    }

    pub async fn get_user(&self) -> Result<User, ReqwestErr> {
        let function = Functions::GetUserData.new();

        let url = format!("{}{}{}{}",
            self.base_url,
            self.token,
            format!("&wsfunction={}", function),
            self.format
        );

        let response = self.client.get(&url).send().await?.json::<User>().await?;
        Ok(response)
    }

    pub async fn get_courses(&self) -> Result<Vec<Course>, ReqwestErr> {
        let function = Functions::GetAllCourses.new();

        let url = format!("{}{}{}{}{}",
            self.base_url,
            self.token,
            format!("&wsfunction={}", function),
            self.format,
            self.user_id.clone().unwrap_or_default()
        );

        let response = self.client.get(&url).send().await?.json::<Vec<Course>>().await?;
        Ok(response)
    }

    pub async fn get_grades(&self) -> Result<Grades, ReqwestErr> {
        let function = Functions::GetGrades.new();

        let url = format!("{}{}{}{}{}{}",
            self.base_url,
            self.token,
            format!("&wsfunction={}", function),
            self.format,
            self.user_id.clone().unwrap_or_default(),
            self.course_id.clone().unwrap_or_default()
        );


        let response = self.client.get(&url).send().await?.json::<Grades>().await?;
        Ok(response)
    }

    pub async fn get_deadlines(&self) -> Result<Events, ReqwestErr> {
        let function = Functions::GetDeadlines.new();

        let url = format!("{}{}{}{}",
            self.base_url,
            self.token,
            format!("&wsfunction={}", function),
            self.format
        );

        let response = self.client.get(&url).send().await?.json::<Events>().await?;
        Ok(response)
    }
}