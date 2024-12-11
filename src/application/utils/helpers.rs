use actix_web::web::Json;
use chrono::{NaiveDateTime, NaiveTime, ParseError, TimeZone, Timelike, Utc};
use std::{error, fmt::Debug, ptr::null};
use regex::Regex;
use crate::domain::{course::Course, user::User};
use serde_json::{error::Error as JsonError, Value};
use jsondiff::diff;



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

pub fn extract_time(date_str: &str) -> Option<String> {
    let re = Regex::new(r"\b(\d{1,2}:\d{2})\b").expect("Failed to create regex");
    if let Some(captures) = re.captures(date_str) {
        Some(captures.get(1)?.as_str().to_string())
    } else {
        None
    }
}

pub fn parse_time_to_seconds(time_str: &str) -> Result<i64, ParseError> {
    let format = "%H:%M";
    let naive_time = NaiveTime::parse_from_str(time_str, format)?;
    let seconds = naive_time.num_seconds_from_midnight() as i64;
    Ok(seconds)
}