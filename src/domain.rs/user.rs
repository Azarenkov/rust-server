use serde::Deserialize;

#[derive(Debug, Deserialize)]

pub struct User {
    pub username: String,
    pub userid: i64,
    pub fullname: String,
}