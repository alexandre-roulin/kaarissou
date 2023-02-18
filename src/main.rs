#![feature(once_cell)]
#![feature(drain_filter)]

pub(crate) mod config;
pub(crate) mod constant;
mod error;
mod framework;
pub(crate) mod logger;
mod model;
pub(crate) mod stalker;

use async_trait::async_trait;
use config::Config;
use constant::{KRYSSOU, KRYSTALINO_SERVER, PRIV_CHANNEL};
use error::KaarissouError;
use framework::*;
use logger::log_voice_channel;
use model::KaarissouUser;
use poise::serenity_prelude::{self as serenity, GatewayIntents, Http, Ready, VoiceState};
use poise::serenity_prelude::{Mutex, UserId};
use songbird::driver::DecodeMode;
use songbird::{
    events::context_data::SpeakingUpdateData, model::payload::Speaking, Event, EventContext,
    SerenityInit,
};

use std::{collections::HashMap, sync::Arc};
use tokio::time::Instant;

type Context<'a> = poise::Context<'a, Data, KaarissouError>;
type Data = Handler;

#[derive(Clone, Debug)]
pub struct Handler(pub Arc<Mutex<Inner>>);

#[derive(Debug)]
pub struct Inner {
    users_channel: HashMap<UserId, KaarissouUser>,
    http: Option<Arc<Http>>,
}

#[async_trait]
impl songbird::EventHandler for Handler {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        let inner = &mut *self.0.lock().await;
        let map = &mut inner.users_channel;
        match ctx {
            EventContext::SpeakingStateUpdate(Speaking {
                ssrc,
                user_id: Some(user_id),
                ..
            }) => {
                if let Some(kuser) = map.get_mut(&UserId(user_id.0)) {
                    kuser.ssrc = Some(*ssrc)
                }
            }
            EventContext::SpeakingUpdate(SpeakingUpdateData { speaking, ssrc, .. }) => {
                if let Some(kuser) = map.values_mut().find(|v| v.ssrc == Some(*ssrc)) {
                    if !*speaking {
                        // If he is not speaking, he is silent :)
                        kuser.silent_since = Instant::now()
                    }
                }
            }
            EventContext::Track(_) => {
                let http = inner.http.as_ref().unwrap().clone();
                for uid in map
                    .iter()
                    .filter(|(_, v)| v.cid == PRIV_CHANNEL)
                    .map(|(k, _)| k)
                {
                    KRYSTALINO_SERVER
                        .move_member(&http, uid, PRIV_CHANNEL)
                        .await
                        .unwrap();
                }
            }
            EventContext::VoicePacket(packet) => {
                if let (Some(audio), Some(kuser)) = (
                    packet.audio.as_ref(),
                    map.values_mut()
                        .find(|v| v.ssrc == Some(packet.packet.ssrc)),
                ) {
                    if let Some(recorder) = kuser.recorder.as_mut() {
                        for b in audio {
                            let _ = recorder.writer.write_sample(*b);
                        }
                    }
                }
            }
            &_ => todo!(),
        }
        None
    }
}

async fn event_listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    framework: poise::FrameworkContext<'_, Data, KaarissouError>,
    _user_data: &Data,
) -> Result<(), KaarissouError> {
    match event.clone() {
        poise::Event::Ready { data_about_bot } => {
            ready(framework.user_data.clone(), ctx, data_about_bot).await;
        }
        poise::Event::VoiceStateUpdate { old, new } => {
            voice_state_update(framework.user_data.clone(), ctx, old, new).await;
        }
        _ => {}
    }

    Ok(())
}

async fn ready(_: Data, _: &serenity::Context, _: Ready) {
    // data.0.lock().await.http = Some(ctx.http.clone());
    // let manager = songbird::get(ctx).await.unwrap();
    // let (handler, result) = manager.join(KRYSTALINO_SERVER, PRIV_CHANNEL).await;
    // if result.is_ok() {
    //     let mut handler = handler.lock().await;
    //     handler.add_global_event(CoreEvent::SpeakingStateUpdate.into(), data.clone());
    //     handler.add_global_event(CoreEvent::SpeakingUpdate.into(), data.clone());
    //     handler.add_global_event(CoreEvent::VoicePacket.into(), data.clone());
    // }
}

async fn voice_state_update(
    handler: Handler,
    ctx: &serenity::Context,
    _: Option<VoiceState>,
    mut vs: VoiceState,
) {
    if vs.guild_id != Some(KRYSTALINO_SERVER) {
        return;
    }

    let inner = &mut *handler.0.lock().await;
    let map = &mut inner.users_channel;
    let uid = vs.user_id;

    // If Kryssou is mute or deaf, let's assume that he is not there.
    let kryssou_user = map.get(&KRYSSOU).cloned();

    let is_away = vs.self_deaf || vs.self_mute;
    let member = vs.member.as_mut().unwrap();

    // Update the map and log !
    match (is_away, vs.channel_id) {
        // If the user is muted (true) or leave the discord (None).
        (false, None) | (true, None) | (true, Some(_)) => {
            map.remove(&uid);
        }
        // User join a new voice channel and is not mute or deaf!
        (false, Some(cid)) => {
            let first_entry = map.insert(uid, KaarissouUser::new(cid)).is_none();
            if kryssou_user.map(|kuser| kuser.cid) != Some(cid) && first_entry {
                log_voice_channel(ctx, uid, cid).await;
            }
        }
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 12)]
async fn main() {
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let config_str = tokio::fs::read_to_string("./config.toml").await.unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();

    poise::Framework::builder()
        .token(&config.token)
        .intents(intents)
        .user_data_setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Handler(Arc::new(Mutex::new(Inner {
                    users_channel: HashMap::new(),
                    http: None,
                }))))
            })
        })
        .options(poise::FrameworkOptions {
            commands: vec![register_application(), status(), log(), hash(), stalker(), train()],
            on_error: |err| Box::pin(error_handler(err)),
            listener: |ctx, event, framework, user_data| {
                Box::pin(event_listener(ctx, event, framework, user_data))
            },
            // This is also where commands go
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("!".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                )),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .client_settings(|c| {
            c.register_songbird_from_config(
                songbird::Config::default().decode_mode(DecodeMode::Decode),
            )
        })
        .run()
        .await
        .expect("can't create framework");
}
