use poise::serenity_prelude::{ChannelId, Context, Mention, UserId};

use crate::{
    constant::{KRYSSOU, LOG_CHANNEL},
};

pub async fn log_voice_channel(
    ctx: &Context,
    uid: UserId,
    cid: ChannelId,
) {
    if uid != KRYSSOU {
        let _ = LOG_CHANNEL
            .say(
                &ctx,
                format!(
                    "{} is connected to {}",
                    Mention::from(uid),
                    Mention::from(cid)
                ),
            )
            .await;
    }
}
