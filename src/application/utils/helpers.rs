use chrono::{NaiveTime, ParseError, Timelike};
use serde::Serialize;
use tokio::{sync::mpsc::Sender, task};
use regex::Regex;
use crate::adapters::messaging::fcm_adapter::FcmAdapter;

pub fn extract_date_and_time(html: &str) -> Option<String> {
    let re = Regex::new(r#"<a href="[^"]+">([^<]+)</a>, (\d{2}:\d{2})"#).expect("Failed to create regex");
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

pub fn tx_sender(message: FcmAdapter, tx: Sender<FcmAdapter>) {
    task::spawn(async move {
        if let Err(e) = tx.send(message).await {
            eprintln!("Failed to send message to channel: {:?}", e);
        }
    });
}

pub fn compare_objects<T: Clone + Serialize>(data: T, db_data: T) -> Option<T> {
    let data_json = serde_json::to_string(&data).unwrap();
    let db_data_json = serde_json::to_string(&db_data).unwrap();

    if data_json != db_data_json {
        Some(data)
    } else {
        None
    }
}