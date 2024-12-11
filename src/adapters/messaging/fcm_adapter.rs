use fcm::message::{Message, Notification, Target};
use fcm::response::FcmResponse;
use fcm::{FcmClient, FcmClientError};
use serde_json::json;
use crate::application::repositories::fcm_abstract::FcmRepositoryAbstract;

pub struct FcmAdapter {
    pub message: Message
}

impl FcmAdapter {
    pub fn new(device_token: &String, title: &String, body: &String, old_body: Option<&String>) -> Self {
        let message;

        if let Some(old_body) = old_body {
            message = Message {
                data: Some(json!({
                   "message": "Howdy!",
                })),
                notification: Some(Notification {
                    title: Some(title.to_string()),
                    body: Some(format!("{} {}", old_body, body)),
                    image: None,
                }),
                target: Target::Token(device_token.to_string()),
                android: None,
                webpush: None,
                apns: None,
                fcm_options: None,
                };
        } else {
            message = Message {
                data: Some(json!({
                   "message": "Howdy!",
                })),
                notification: Some(Notification {
                    title: Some(title.to_string()),
                    body: Some(format!("{}", body)),
                    image: None,
                }),
                target: Target::Token(device_token.to_string()),
                android: None,
                webpush: None,
                apns: None,
                fcm_options: None,
                };
        }

        FcmAdapter { message: message }
    }
}

