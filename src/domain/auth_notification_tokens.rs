
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Tokens {
    pub token: String,
    pub device_token: Option<String>,
    pub tg_token: Option<String>
}