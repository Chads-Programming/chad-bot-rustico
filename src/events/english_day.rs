use chrono::{DateTime, Datelike, Utc};
use chrono_tz::America::Argentina::Buenos_Aires;
use serenity::all::{Context, Message};
use tracing::{error, info};

use crate::{consts, helpers};

pub async fn handle(ctx: &Context, msg: &Message) {
    let utc_now: DateTime<Utc> = Utc::now();

    let ba_now = utc_now.with_timezone(&Buenos_Aires);

    if ba_now.weekday() != chrono::Weekday::Fri {
        return;
    }

    if helpers::langs::check_english_lang(msg.content.as_str()) {
        return;
    }

    let message = format!(
        "Today is the english day, please try to send your text messages in english {}",
        consts::DUDE_EMOJI
    );

    if let Err(err) = msg.reply(&ctx.http, message).await {
        error!("Error on intercept message: {err:?}");

        return;
    }

    info!("English day message replied");
}
