
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Tokens {
    pub token: String,
    pub device_token: Option<String>,
    pub tg_token: Option<String>
}