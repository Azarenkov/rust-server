use regex::Regex;
use crate::domain::{course::Course, user::User};
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

pub fn compare(data: String, db_data: String) -> bool {
    if data != db_data {
        false
    } else {
        true
    }
}