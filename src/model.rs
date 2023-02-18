use hound::{ WavWriter};
use poise::serenity_prelude::ChannelId;
use std::{fs::File, io::BufWriter};
use tokio::time::Instant;
pub struct Recorder {
    pub writer: WavWriter<BufWriter<File>>,
    pub filename: String,
}
pub struct KaarissouUser {
    pub ssrc: Option<u32>,
    pub cid: ChannelId,
    pub silent_since: Instant,
    pub recorder: Option<Recorder>,
}

impl Clone for KaarissouUser {
    fn clone(&self) -> Self {
        Self {
            ssrc: self.ssrc,
            cid: self.cid,
            silent_since: self.silent_since,
            recorder: None,
        }
    }
}

impl std::fmt::Debug for KaarissouUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("KaarissouUser")
            .field("ssrc", &self.ssrc)
            .field("cid", &self.cid)
            .field("silent_since", &self.silent_since)
            .field("writer", &"ignored_field")
            .finish()
    }
}

impl KaarissouUser {
    pub fn new(cid: ChannelId) -> Self {
        Self {
            ssrc: None,
            cid,
            silent_since: Instant::now(),
            recorder: None,
        }
    }

    // We define afk by not speaking since 2 minutes
    // pub fn is_afk(&self) -> bool {
    //     Instant::now().duration_since(self.silent_since) > Duration::from_secs(60)
    // }
}
