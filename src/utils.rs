use crate::constant::SNAPCHAT_CHANNEL;
use poise::serenity_prelude::{Context, UserId};

pub async fn remove_all_messages(ctx: &Context, uid: UserId, old: bool) -> bool {
    let messages = SNAPCHAT_CHANNEL
        .messages(ctx, |g| {
            g.limit(100);
            g
        })
        .await
        .unwrap()
        .into_iter()
        .filter(|msg| msg.author.id == uid)
        .map(|msg| msg.id)
        .collect::<Vec<_>>();
    //  Try again if there are more messages to delete.
    let retry = !messages.is_empty();
    if old {
        for msg in messages {
            let _ = SNAPCHAT_CHANNEL.delete_message(ctx, msg).await;
        }
    } else {
        let _ = SNAPCHAT_CHANNEL.delete_messages(ctx, messages).await;
    }
    retry
}
