mod config;
mod constant;

use crate::constant::SNAPCHAT_CHANNEL;
use async_trait::async_trait;
use config::Config;
use constant::{KRYSSOU, LOG_CHANNEL, ROUND_TABLE_CHANNEL, SNAPCHAT_ROLE};
use serenity::{
    model::{
        prelude::{ChannelId, GatewayIntents, Mention, UserId},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler, Mutex},
    Client,
};
use std::{collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct Handler(pub Arc<Mutex<Inner>>);

#[derive(Default)]
pub struct Inner {
    users_channel: HashMap<u64, u64>,
}

async fn remove_all_messages(ctx: &Context, uid: u64) -> bool {
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
    let retry = !(messages.is_empty() || messages.len() > 100);
    let _ = chid.delete_messages(ctx, messages).await;
    retry
}

#[async_trait]
impl EventHandler for Handler {
    async fn voice_state_update(
        &self,
        ctx: Context,
        _: Option<VoiceState>,
        voice_state: VoiceState,
    ) {
        let inner = &mut *self.0.lock().await;
        let map = &mut inner.users_channel;
        let uid = voice_state.user_id.0;
        let kryssou_chan = map.get(&KRYSSOU).cloned();
        let cid = if let Some(cid) = voice_state.channel_id {
            cid.0
        } else {
            // User leave the Discord !
            map.remove(&uid);
            return;
        };

        let mut member = voice_state.member.unwrap();

        // User is not `KRYSSOU` and channel is the same as Kryssou channel
        if uid != KRYSSOU && kryssou_chan == Some(cid) {
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

        // Update the map with new channel id
        map.insert(uid, cid);
        if cid == ROUND_TABLE_CHANNEL {
            let _ = member.add_role(&ctx, SNAPCHAT_ROLE).await;
        } else {
            let _ = member.remove_role(&ctx, SNAPCHAT_ROLE).await;
            while remove_all_messages(&ctx, uid).await {}
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
    let intents = GatewayIntents::non_privileged();
    let config_str = tokio::fs::read_to_string("./config.toml").await.unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();
    let mut client = Client::builder(config.token, intents)
        .event_handler(Handler::default())
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
