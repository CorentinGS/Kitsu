use crate::models::user::KitsuUser;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn update_user(url: String, user: KitsuUser) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .put(url)
        .json(&user)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}

async fn post_user(url: String, user: KitsuUser) -> Result<i8> {
    let new_post: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&user)
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

pub async fn fetch_user(url: String, mut kitsu_user: KitsuUser) -> Result<i8> {
    let echo_json: serde_json::Value = reqwest::get(&url).await?.json().await?;
    let id: i8;
    if echo_json["success"].to_string().parse::<bool>().unwrap() == true {
        id = echo_json["data"]["ID"].to_string().parse::<i8>().unwrap();
        if echo_json["data"]["user_name"].to_string() != kitsu_user.user_name {
            kitsu_user.vip = echo_json["data"]["vip"]
                .to_string()
                .parse::<bool>()
                .unwrap();
            let _ = update_user(
                format!("http://127.0.0.1:1812/api/v1/guild/id/{:?}", id),
                kitsu_user,
            )
            .await;
        }
    } else {
        id = post_user(
            "http://127.0.0.1:1812/api/v1/user/new".to_string(),
            kitsu_user,
        )
        .await
        .unwrap();
    }

    Ok(id)
}
