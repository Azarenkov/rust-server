use regex::Regex;
use crate::domain::user::User;
use serde_json::error::Error as JsonError;



pub fn extract_link_and_date(html: &str) -> Option<String> {
    let re = Regex::new(r#"<a class="dimmed" href="[^"]+">([^<]+)</a>, ([^<]+)</span>"#).expect("Failed to create regex");
    if let Some(captures) = re.captures(html) {
        let date = captures.get(1)?.as_str().to_string();
        let time = captures.get(2)?.as_str().to_string();
        Some(format!("{} {}", date, time))
    } else {
        None
    }
}

pub trait Compare {
    fn compare(&self, db_data: String) -> Result<bool, JsonError>;
}

impl Compare for User {
    fn compare(&self, db_data: String) -> Result<bool, JsonError> {
        let user_value = serde_json::to_string(&self).map_err(|e| e)?;

        if user_value != db_data {
            Ok(false)
        } else {
            Ok(true)
        }
    }
}