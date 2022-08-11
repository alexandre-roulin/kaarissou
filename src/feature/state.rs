//git

use std::process::Stdio;

use serenity::{model::prelude::ChannelId, prelude::Context};
use tokio::process::Command;

use crate::{
    constant::{KAARISSOU, SNAPCHAT_CHANNEL},
    utils::remove_all_messages,
};

pub async fn state() -> String {
    let hash = String::from_utf8(
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
    .unwrap();

    let status = String::from_utf8(
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
    .unwrap();

    let msg = String::from_utf8(
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
    .unwrap();
    let source_code = format!("Source code is avalaible on {}.", "https://github.com/alexandre-roulin/kaarissou");
    let status_fmt = format!("```{}```", status.as_str());
    [
        source_code.as_str(),
        "Last update:",
        msg.as_str(),
        "Current hash:",
        hash.as_str(),
        "Current status:",
        if status.as_str().is_empty() {
            "Nothing different than github"
        } else { 
            status_fmt.as_str()
        },
    ]
    .join("\n")
}

pub async fn print_state(ctx: &Context) {
    remove_all_messages(ctx, KAARISSOU, true).await;
    let _ = ChannelId(SNAPCHAT_CHANNEL).say(ctx, state().await).await;
}
