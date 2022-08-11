use crate::constant::{KRYSSOU, LOG_CHANNEL};
use serenity::{
    model::prelude::{ChannelId, Mention, UserId},
    prelude::Context,
};

pub async fn log_voice_channel(ctx: &Context, uid: u64, cid: u64, kryssou_chan: Option<u64>) {
    // User is not `KRYSSOU` and channel is not the same as Kryssou channel
    if uid != KRYSSOU && kryssou_chan != Some(cid) {
        let _ = ChannelId(LOG_CHANNEL)
            .say(
                &ctx,
                format!(
                    "{} is connected to {}",
                    Mention::from(UserId(uid)),
                    Mention::from(ChannelId(cid))
                ),
            )
            .await;
    }
}
