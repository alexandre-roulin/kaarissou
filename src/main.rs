pub(crate) mod config;
pub(crate) mod constant;
pub(crate) mod feature;
mod model;
pub(crate) mod utils;

use async_trait::async_trait;
use config::Config;
use constant::{KRYSSOU, KRYSTALINO_SERVER, PRIV_CHANNEL, SNAPCHAT_ROLE};
use feature::{logger::log_voice_channel, state::print_state};
use model::KaarissouUser;
use serenity::{
    model::{
        prelude::{GatewayIntents, Ready, UserId},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler, Mutex},
    Client,
};
use songbird::{
    events::context_data::SpeakingUpdateData, model::payload::Speaking, CoreEvent, Event,
    EventContext, EventHandler as VoiceEventHandler, SerenityInit,
};
use std::{collections::HashMap, sync::Arc};
use tokio::time::Instant;
use utils::remove_all_messages;

#[derive(Clone)]
pub struct Handler(pub Arc<Mutex<Inner>>);

pub struct Inner {
    users_channel: HashMap<UserId, KaarissouUser>,
}

#[async_trait]
impl VoiceEventHandler for Handler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let inner = &mut *self.0.lock().await;
        let map = &mut inner.users_channel;
        match ctx {
            EventContext::SpeakingStateUpdate(Speaking { ssrc, user_id, .. }) => {
                if let Some(uid) = user_id {
                    if let Some(kuser) = map.get_mut(&UserId(uid.0)) {
                        kuser.ssrc = Some(*ssrc)
                    }
                }
            }
            EventContext::SpeakingUpdate(SpeakingUpdateData { speaking, ssrc, .. }) => {
                if let Some(kuser) = map.values_mut().find(|v| v.ssrc == Some(*ssrc)) {
                    if *speaking {
                        kuser.silent_since = Instant::now()

                    }
                }
            }
            _ => {}
        }
        None
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        let manager = songbird::get(&ctx).await.unwrap();
        let (handler, result) = manager.join(KRYSTALINO_SERVER, PRIV_CHANNEL).await;
        if result.is_ok() {
            let mut handler = handler.lock().await;
            handler.add_global_event(CoreEvent::SpeakingStateUpdate.into(), self.clone());
            handler.add_global_event(CoreEvent::SpeakingUpdate.into(), self.clone());
        }

        print_state(&ctx).await;
    }

    async fn voice_state_update(&self, ctx: Context, _: Option<VoiceState>, vs: VoiceState) {
        if vs.guild_id != Some(KRYSTALINO_SERVER) {
            return;
        }

        let inner = &mut *self.0.lock().await;
        let map = &mut inner.users_channel;
        let uid = vs.user_id;

        // If Kryssou is mute or deaf, let's assume that he is not there.
        let kryssou_user = map.get(&KRYSSOU);

        let is_away = vs.self_deaf || vs.self_mute;
        let mut member = vs.member.unwrap();

        // Update the map and log !
        match (is_away, vs.channel_id) {
            // If the user is muted (true) or leave the discord (None).
            (false, None) | (true, None) | (true, Some(_)) => {
                map.remove(&uid.into());
                let _ = member.remove_role(&ctx, SNAPCHAT_ROLE).await;
                while remove_all_messages(&ctx, uid, false).await {}
            }
            // User join a new voice channel and is not mute or deaf!
            (false, Some(cid)) => {
                log_voice_channel(&ctx, uid, cid, kryssou_user).await;
                map.insert(uid, KaarissouUser::new(cid));
                if cid == PRIV_CHANNEL {
                    let _ = member.add_role(&ctx, SNAPCHAT_ROLE).await;
                }
            }
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let config_str = tokio::fs::read_to_string("./config.toml").await.unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();
    let mut client = Client::builder(config.token, intents)
        .event_handler(Handler(Arc::new(Mutex::new(Inner {
            users_channel: HashMap::new(),
        }))))
        .register_songbird()
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
