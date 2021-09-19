use crate::models::channel::KitsuChannel;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn update_channel(url: String, chan: KitsuChannel) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .put(url)
        .json(&chan)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}

async fn post_channel(url: String, channel: KitsuChannel) -> Result<i8> {
    let new_post: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&channel)
        .send()
        .await?
        .json()
        .await?;
    let id: i8;
    if new_post["success"].to_string().parse::<bool>().unwrap() == true {
        id = new_post["data"]["ID"].to_string().parse::<i8>().unwrap();
    } else {
        id = 0;
    }

    Ok(id)
}

pub async fn fetch_channel(url: String, kitsu_channel: KitsuChannel) -> Result<i8> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let id: i8;
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        id = echo_json["data"]["ID"].to_string().parse::<i8>().unwrap();
        if echo_json["data"]["channel_name"].to_string() != kitsu_channel.channel_name {
            let _ = update_channel(
                format!("http://127.0.0.1:1812/api/v1/channel/id/{:?}", id),
                kitsu_channel,
            )
            .await;
        }
    } else {
        id = post_channel(
            "http://127.0.0.1:1812/api/v1/channel/new".to_string(),
            kitsu_channel,
        )
        .await
        .unwrap();
    }

    Ok(id)
}
