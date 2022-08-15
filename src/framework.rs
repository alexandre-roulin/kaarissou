use tokio::process::Command;

use crate::constant::SNAPCHAT_CHANNEL;
use crate::Context;
use crate::{Data, KaarissouError};
use std::process::Stdio;

#[poise::command(prefix_command, slash_command)]
pub async fn status(ctx: Context<'_>) -> Result<(), KaarissouError> {
    if ctx.channel_id() != SNAPCHAT_CHANNEL {
        return Err(KaarissouError::InvalidChannel {
            wrong: ctx.channel_id(),
            right: SNAPCHAT_CHANNEL,
        });
    }

    ctx.say(
        String::from_utf8(
            Command::new("git")
                .args(&["status", "-s"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .await
                .unwrap()
                .stdout,
        )
        .unwrap(),
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn hash(ctx: Context<'_>) -> Result<(), KaarissouError> {
    if ctx.channel_id() != SNAPCHAT_CHANNEL {
        return Err(KaarissouError::InvalidChannel {
            wrong: ctx.channel_id(),
            right: SNAPCHAT_CHANNEL,
        });
    }

    ctx.say(
        String::from_utf8(
            Command::new("git")
                .args(&["log", "--pretty=format:\"%h\"", "-n", "1"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .await
                .unwrap()
                .stdout,
        )
        .unwrap(),
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn log(ctx: Context<'_>) -> Result<(), KaarissouError> {
    if ctx.channel_id() != SNAPCHAT_CHANNEL {
        return Err(KaarissouError::InvalidChannel {
            wrong: ctx.channel_id(),
            right: SNAPCHAT_CHANNEL,
        });
    }

    ctx.say(
        String::from_utf8(
            Command::new("git")
                .args(&["log", "--pretty=format:\"%s\"", "-n", "1"])
                .stdout(Stdio::piped())
                .spawn()
                .unwrap()
                .wait_with_output()
                .await
                .unwrap()
                .stdout,
        )
        .unwrap(),
    )
    .await?;
    Ok(())
}

#[poise::command(prefix_command, required_permissions = "ADMINISTRATOR")]
pub async fn register_application(ctx: Context<'_>) -> Result<(), KaarissouError> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}

pub async fn error_handler(error: poise::FrameworkError<'_, Data, KaarissouError>) {
    eprintln!("Oh noes, we got an error: {:#?}", error);
}
