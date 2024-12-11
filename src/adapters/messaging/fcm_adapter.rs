use fcm::message::{Message, Notification, Target};

pub struct FcmAdapter {
    pub message: Message
}

impl FcmAdapter {
    pub fn new(device_token: &String, title: &String, body: &String, old_body: Option<&String>) -> Self {
        let message;

        if let Some(old_body) = old_body {
            message = Message {
                data: None,
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
                data: None,
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

impl AsRef<Message> for FcmAdapter {

    fn as_ref(&self) -> &Message {

        &self.message

    }

}

