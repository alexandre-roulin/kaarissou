use poise::serenity_prelude::{ChannelId, Mention};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KaarissouError {
    #[error("serenity error: {0}")]
    Disconnect(#[from] poise::serenity_prelude::Error),
    #[error("invalid channel (expected {}, found {})", Mention::from(*wrong), Mention::from(*right))]
    InvalidChannel { wrong: ChannelId, right: ChannelId },
}
