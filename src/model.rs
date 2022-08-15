use poise::serenity_prelude::ChannelId;
use std::time::Duration;
use tokio::time::Instant;

#[derive(Clone, Debug)]
pub struct KaarissouUser {
    pub ssrc: Option<u32>,
    pub cid: ChannelId,
    pub silent_since: Instant,
}

impl KaarissouUser {
    pub fn new(cid: ChannelId) -> Self {
        Self {
            ssrc: None,
            cid,
            silent_since: Instant::now(),
        }
    }

    /// We define afk by not speaking since 2 minutes
    pub fn is_afk(&self) -> bool {
        Instant::now().duration_since(self.silent_since) > Duration::from_secs(60)
    }
}
