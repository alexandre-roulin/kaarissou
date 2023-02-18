use poise::serenity_prelude::{ChannelId, Mention, SerenityError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KaarissouError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("hound error: {0}")]
    Hound(#[from] hound::Error),
    #[error("serenity error: {0}")]
    Serenity(#[from] SerenityError),
    #[error("invalid channel (expected {}, found {})", Mention::from(*wrong), Mention::from(*right))]
    InvalidChannel { wrong: ChannelId, right: ChannelId },
    #[error("reqwest_middleware error: {0}")]
    ReqwestMiddleWare(#[from] reqwest_middleware::Error),
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    
    
}
