pub(crate) mod config;
pub(crate) mod constant;
pub(crate) mod feature;
pub(crate) mod utils;

use async_trait::async_trait;
use config::Config;
use constant::{KRYSSOU, KRYSTALINO_SERVER, PRIV_CHANNEL, SNAPCHAT_ROLE};
use feature::{state::print_state, logger::log_voice_channel};
use serenity::{
    model::{
        prelude::{GatewayIntents, GuildId, Ready},
        voice::VoiceState,
    },
    prelude::{Context, EventHandler, Mutex},
    Client,
};
use std::{collections::HashMap, sync::Arc};
use utils::remove_all_messages;

#[derive(Default)]
pub struct Handler(pub Arc<Mutex<Inner>>);

#[derive(Default)]
pub struct Inner {
    users_channel: HashMap<u64, u64>,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _data_about_bot: Ready) {
        print_state(&ctx).await;
    }

    async fn voice_state_update(
        &self,
        ctx: Context,
        _: Option<VoiceState>,
        voice_state: VoiceState,
    ) {
        if voice_state.guild_id != Some(GuildId(KRYSTALINO_SERVER)) {
            return;
        }

        let inner = &mut *self.0.lock().await;
        let map = &mut inner.users_channel;
        let uid = voice_state.user_id.0;
        let kryssou_chan = map.get(&KRYSSOU).cloned();
        let cid = voice_state.channel_id.map(|c| c.0);

        // Update the map and log !
        if let Some(cid) = cid {
            // User join a new voice channel !
            log_voice_channel(&ctx, uid, cid, kryssou_chan).await;
            map.insert(uid, cid);
        } else {
            // User leave the Discord !
            map.remove(&uid);
        };

        let mut member = voice_state.member.unwrap();

        // Manage role for snapchat !
        if cid == Some(PRIV_CHANNEL) {
            let _ = member.add_role(&ctx, SNAPCHAT_ROLE).await;
        } else {
            let _ = member.remove_role(&ctx, SNAPCHAT_ROLE).await;
            while remove_all_messages(&ctx, uid, false).await {}
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
