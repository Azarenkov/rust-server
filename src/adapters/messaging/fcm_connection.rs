use fcm_rs::client::FcmClient;
use fcm_rs::error::FcmError;

pub async fn get_messaging_service(path: String) -> Result<FcmClient, FcmError> {
    let client = FcmClient::new(&path).await?;
    Ok(client)
}