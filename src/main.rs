mod general;
mod models;

use general::ping::*;
use models::{channel::KitsuChannel, guild::KitsuGuild, message::KitsuMessage, user::KitsuUser};

use chrono::{DateTime, Utc};
use dotenv::dotenv;

use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{
        channel::Message, event::ResumedEvent, gateway::Ready, id::GuildId, prelude::Activity,
    },
    prelude::*,
};
use std::{collections::HashSet, env};
struct Handler;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn register_message(ctx: Context, msg: Message) {
    let now: DateTime<Utc> = Utc::now();

    let kitsu_guild = KitsuGuild {
        guild_id: msg.guild_id.unwrap().0.to_string(),
        guild_name: GuildId(msg.guild_id.unwrap().0)
            .name(&ctx.cache)
            .await
            .unwrap(),
        vip: false,
    };

    let guild_id = &fetch_guild(
        format!(
            "http://127.0.0.1:1812/api/v1/guild/guildId/{:?}",
            msg.guild_id.unwrap().0
        ),
        kitsu_guild,
    )
    .await
    .unwrap();
    if *guild_id == 0 {
        return;
    }

    let kitsu_user = KitsuUser {
        user_id: msg.author.id.0.to_string(),
        user_name: msg.author.name,
        vip: false,
    };

    let user_id = &fetch_user(
        format!(
            "http://127.0.0.1:1812/api/v1/user/userId/{:?}",
            msg.author.id.0
        ),
        kitsu_user,
    )
    .await
    .unwrap();

    if *user_id == 0 {
        return;
    }

    let kitsu_channel = KitsuChannel {
        channel_id: msg.channel_id.0.to_string(),
        channel_name: msg.channel_id.name(&ctx.cache).await.unwrap(),
        guild_id: *guild_id,
        ignore: false,
    };

    let channel_id = &fetch_channel(
        format!(
            "http://127.0.0.1:1812/api/v1/channel/channelId/{:?}",
            msg.channel_id.0
        ),
        kitsu_channel,
    )
    .await
    .unwrap();

    if *channel_id == 0 {
        return;
    }

    let kitsu_message = KitsuMessage {
        guild_id: *guild_id,
        user_id: *user_id,
        message_id: msg.id.to_string(),
        channel_id: *channel_id,
        date_id: now.format("%Y%m%d").to_string().parse::<i64>().unwrap(),
    };

    let _ = post_url(
        "http://127.0.0.1:1812/api/v1/message/new".to_string(),
        kitsu_message,
    )
    .await;
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        if let Some(_) = msg.guild_id {
            register_message(ctx, msg).await;
        } else {
            return;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
        ctx.online().await;
        ctx.set_activity(Activity::playing("Kitsu ! Generating stats"))
            .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
}

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

async fn post_url(url: String, msg: KitsuMessage) -> Result<()> {
    let _: serde_json::Value = reqwest::Client::new()
        .post(url)
        .json(&msg)
        .send()
        .await?
        .json()
        .await?;

    Ok(())
}

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

async fn fetch_guild(url: String, mut kitsu_guild: KitsuGuild) -> Result<i8> {
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

async fn fetch_channel(url: String, kitsu_channel: KitsuChannel) -> Result<i8> {
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

async fn fetch_user(url: String, mut kitsu_user: KitsuUser) -> Result<i8> {
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

#[group]
#[commands(ping)]
struct General;

#[tokio::main]
async fn main() {
    // Setup dotenv
    dotenv().ok().expect(".env file not found");

    // Configure the client with your Discord bot token in the environment.
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let http = Http::new_with_token(&token);
    let (owners, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Create the framework
    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("~"))
        .group(&GENERAL_GROUP);

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .framework(framework)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
