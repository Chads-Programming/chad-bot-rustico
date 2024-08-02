use chrono::{DateTime, Datelike, Utc};
use chrono_tz::America::Argentina::Buenos_Aires;
use regex::Regex;
use serenity::all::{Context, Message};
use tracing::{error, info};

use crate::{consts, helpers};

pub async fn handle(ctx: &Context, msg: &Message) {
    let utc_now: DateTime<Utc> = Utc::now();

    let ba_now = utc_now.with_timezone(&Buenos_Aires);

    if consts::ENGLISH_DAY_WHITELIST.contains(&msg.author.id.into()) {
        return;
    }

    if !consts::ENGLISH_DAYS.contains(&ba_now.weekday()) {
        return;
    }

    let content = extract_text_without_urls(msg.content.as_str());

    if content.is_empty() {
        return;
    }

    if helpers::langs::check_english_lang(content.as_str()) {
        return;
    }

    if msg.channel_id != consts::ENGLISH_CHANNEL_ID {
        return;
    }

    let message = format!(
        "The messages you send through this channel must be in English during English Day <:{}:{}>",
        consts::DUDE_EMOJI.1,
        consts::DUDE_EMOJI.0,
    );

    if let Err(err) = msg.reply(&ctx.http, message).await {
        error!("Error on intercept message: {err:?}");

        return;
    }

    info!("English day message replied");
}

fn extract_text_without_urls(input: &str) -> String {
    let url_regex = Regex::new(r"https?://\S+").unwrap();

    let result = url_regex.replace_all(input, "");

    result.trim().to_string()
}
