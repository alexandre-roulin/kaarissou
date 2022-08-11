use serenity::{model::prelude::ChannelId, prelude::Context};

use crate::constant::SNAPCHAT_CHANNEL;

pub async fn remove_all_messages(ctx: &Context, uid: u64, old: bool) -> bool {
    let chid = ChannelId(SNAPCHAT_CHANNEL);
    let messages = chid
        .messages(ctx, |g| {
            g.limit(100);
            g
        })
        .await
        .unwrap()
        .into_iter()
        .filter(|msg| msg.author.id.0 == uid)
        .map(|msg| msg.id)
        .collect::<Vec<_>>();
    //  Try again if there are more messages to delete.
    let retry = !messages.is_empty();
    if old {
        for msg in messages {
            let _ = chid.delete_message(ctx, msg).await;
        }
    } else {
        let _ = chid.delete_messages(ctx, messages).await;
    }
    retry
}
