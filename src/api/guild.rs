use crate::KitsuGuild;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn post_guild(url: String, guild: KitsuGuild) -> Result<i8> {
    let new_post: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&guild)
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

async fn update_guild(url: String, guild: KitsuGuild) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .put(url)
        .json(&guild)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}

pub async fn fetch_guild(url: String, mut kitsu_guild: KitsuGuild) -> Result<i8> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let mut id: i8;
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        id = echo_json["data"]["ID"].to_string().parse::<i8>().unwrap();

        if echo_json["data"]["vip"]
            .to_string()
            .parse::<bool>()
            .unwrap()
            == false
        {
            id = 0;
        }

        if echo_json["data"]["guild_name"].to_string() != kitsu_guild.guild_name {
            kitsu_guild.vip = echo_json["data"]["vip"]
                .to_string()
                .parse::<bool>()
                .unwrap();
            let _ = update_guild(
                format!("http://127.0.0.1:1812/api/v1/guild/id/{:?}", id),
                kitsu_guild,
            )
            .await;
        }
    } else {
        id = post_guild(
            "http://127.0.0.1:1812/api/v1/guild/new".to_string(),
            kitsu_guild,
        )
        .await
        .unwrap();
    }

    Ok(id)
}
