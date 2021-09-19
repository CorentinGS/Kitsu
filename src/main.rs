mod api;
mod general;
mod models;

use api::{channel::fetch_channel, guild::fetch_guild, message::post_message, user::fetch_user};
use chrono::{DateTime, Utc};
use dotenv::dotenv;
use general::ping::*;
use models::{channel::KitsuChannel, guild::KitsuGuild, message::KitsuMessage, user::KitsuUser};

use serenity::{
    async_trait,
    framework::{standard::macros::group, StandardFramework},
    http::Http,
    model::{
        channel::Message, event::ResumedEvent, gateway::Ready, id::GuildId, prelude::Activity,
    },
    prelude::{Client, Context, EventHandler},
};
use std::{collections::HashSet, env};

struct Handler;

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

    let _ = post_message(
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
        ctx.set_activity(Activity::playing("to generate stats ! ~about"))
            .await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        println!("Resumed");
    }
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
