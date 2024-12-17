use fcm_rs::models::{ Message, Notification };

pub struct FcmAdapter {
    pub message: Message
}

impl FcmAdapter {
    pub fn new(device_token: &String, title: &String, body: &String, old_body: Option<&String>) -> Self {
        let message;

        if let Some(old_body) = old_body {
            message = Message {
                data: None,
                token: Some(device_token.to_string()),
                notification: Some(Notification {
                    title: Some(title.to_string()),
                    body: Some(format!("{} {}", old_body, body)),
                }),
            };
        } else {
            message = Message {
                data: None,
                token: Some(device_token.to_string()),
                notification: Some(Notification {
                    title: Some(title.to_string()),
                    body: Some(format!("{}", body)),
                }),
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

