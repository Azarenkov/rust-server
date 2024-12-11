use fcm::{FcmClientError, FcmClient};

pub async fn get_messaging_service(path: String) -> Result<FcmClient, FcmClientError> {
    let client = fcm::FcmClient::builder()
    .service_account_key_json_path(path)
    .build()
    .await?;
    Ok(client)
}