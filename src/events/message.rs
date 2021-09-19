
use crate::{api::{channel::fetch_channel, message::post_message}, models::message::KitsuMessage};
use serenity::model::prelude::GuildId;
use chrono::{DateTime, Utc};
use serenity::{client::Context, model::channel::Message};

use crate::{api::{guild::fetch_guild, user::fetch_user}, models::{channel::KitsuChannel, guild::KitsuGuild, user::KitsuUser}};

pub async fn register_message(ctx: Context, msg: Message) {
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