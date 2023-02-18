use crate::constant::{SNAPCHAT_CHANNEL, TRAIN_CHANNEL};
use crate::stalker::Stalker;
use crate::Context;
use crate::{Data, KaarissouError};
use once_cell::sync::Lazy;
use poise::serenity_prelude::ReactionType;
use std::process::Stdio;
use tokio::process::Command;

static STALKER: Lazy<Stalker> = Lazy::new(Stalker::new);

#[poise::command(prefix_command, slash_command)]
pub async fn stalker(
    ctx: Context<'_>,
    #[description = "SteamId of players to be stalk"] players: String,
    #[description = "Team name"] team: Option<String>,
) -> Result<(), KaarissouError> {
    ctx.defer().await;
    let players = STALKER
        .reqwest(
            team.unwrap_or_else(|| "shit team".to_owned()),
            players
                .split(|c| !char::is_ascii_digit(&c))
                .filter_map(|s| s.parse::<u32>().ok()),
        )
        .await?;

    let p_name = &players[0].name;
    let p_mmr = &players[0].mmr;
    let p_heroes = &players[0].heroes[..2];
    let p_wl = &players[0].total_games;
    ctx.say(format!("{p_name} {p_mmr} {:?} {:?}", p_heroes, p_wl))
        .await?;

    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn train(
    ctx: Context<'_>,
) -> Result<(), KaarissouError> {
    if ctx.channel_id() != TRAIN_CHANNEL {
        return Ok(())
    }
    let _ = ctx.defer().await;
    let message = ctx.say(
    r#"
Quand es ce que tu est dispo pour cette semaine ? A partir de ~21h
🦀 : Lundi
🐶 : Mardi
🐮 : Mercredi
🐦 : Jeudi
🦆 : Vendredi
🐍 : Samedi
🐰 : Dimanche
    "#).await?.into_message().await?;
    let _ = message.react(ctx.discord(), ReactionType::Unicode("🦀".to_owned())).await?;
    let _ = message.react(ctx.discord(), ReactionType::Unicode("🐶".to_owned())).await?;
    let _ = message.react(ctx.discord(), ReactionType::Unicode("🐮".to_owned())).await?;
    let _ = message.react(ctx.discord(), ReactionType::Unicode("🐦".to_owned())).await?;
    let _ = message.react(ctx.discord(), ReactionType::Unicode("🦆".to_owned())).await?;
    let _ = message.react(ctx.discord(), ReactionType::Unicode("🐍".to_owned())).await?;
    let _ = message.react(ctx.discord(), ReactionType::Unicode("🐰".to_owned())).await?;
    Ok(())
}

/// git status -s
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

///git log --pretty=format:"%h" -n 1
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

/// git log --pretty=format:"%s" -n 1
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

pub async fn error_handler(data: poise::FrameworkError<'_, Data, KaarissouError>) {
    let error = match data {
        poise::FrameworkError::Setup { error } => error,
        poise::FrameworkError::Listener {
            error,
            ctx: _,
            event: _,
            framework: _,
        } => error,
        poise::FrameworkError::Command { error, ctx: _ } => error,
        poise::FrameworkError::DynamicPrefix { error } => error,
        _ => return,
    };
    eprintln!("Oh noes, we got an error :{:?}", error);
}
