use crate::models::message::KitsuMessage;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub async fn post_message(url: String, msg: KitsuMessage) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&msg)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}
