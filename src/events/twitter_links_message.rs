use serenity::all::Context;
use serenity::all::Message;
use tracing::error;
use tracing::info;

use crate::events::utils::is_own_message;

pub async fn handle(ctx: &Context, msg: &Message) {
    if is_own_message(ctx, msg) {
        return;
    }

    let new_content: String = get_new_messages(msg);

    if new_content.is_empty() {
        return;
    }

    if let Err(err) = msg.reply(&ctx.http, new_content).await {
        error!("Error on manage twitter_links_message: {err:?}");

        return;
    }

    info!("Twitter link message replied");
}

fn get_new_messages(msg: &Message) -> String {
    let mut new_content: Vec<String> = vec![];
    let replaced_new_lines = msg.content.replace('\n', " ");
    let words = replaced_new_lines.split_whitespace();
    for word in words {
        let matcher = ["/x.com", "/twitter.com"];
        for twitter_uri in matcher.iter() {
            if !word.contains(twitter_uri) {
                continue;
            }

            let new_url = word.replace(twitter_uri, "/vxtwitter.com");
            if !new_content.contains(&new_url) {
                new_content.push(new_url)
            }
        }
    }

    new_content.join(", ")
}
