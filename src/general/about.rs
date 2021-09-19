use crate::utils::constants::*;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::utils::Color;

#[command]
async fn about(ctx: &Context, msg: &Message) -> CommandResult {
    let kitsu = &ctx.cache.current_user().await;
    msg.channel_id.send_message(&ctx, |m| {
        m.embed(|e| {
            e.thumbnail(kitsu.avatar_url().unwrap());
            e.description(format!(r#"**Kitsu** is a discord bot that generates custom statistics on member and room activity for each guild.
            This allows administrators to have a better visibility on the guild activity, peak and off-peak hours, active and dead days etc...

            Struggling? Check out [our wiki](https://github.com/CorentinGS/Kitsu/wiki)"#));
            e.color(Color::BLURPLE);
            e.title("About");
            e.field("Version", format!("``` {}```", KITSU_VERSION), true);
            e.field("API", format!("``` {} ```", API_VERSION), true);
            e
        })
    }).await?;
    Ok(())
}
