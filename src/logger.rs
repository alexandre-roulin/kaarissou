use poise::serenity_prelude::{ChannelId, Context, Mention, UserId};

use crate::{
    constant::{KRYSSOU, LOG_CHANNEL},
    model::KaarissouUser,
};

pub async fn log_voice_channel(
    ctx: &Context,
    uid: UserId,
    cid: ChannelId,
    kryssou_user: Option<&KaarissouUser>,
) {
    let kryssou_is_afk = kryssou_user.map(KaarissouUser::is_afk).unwrap_or(true);
    // User is not `KRYSSOU` and channel is not the same as Kryssou channel or Kryssou is afk
    if uid != KRYSSOU && (kryssou_user.map(|u| u.cid) != Some(cid) || kryssou_is_afk) {
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
