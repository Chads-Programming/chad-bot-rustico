use serenity::all::Context;
use serenity::all::Message;

use crate::events::utils::is_own_message;

pub async fn handle(ctx: &Context, msg: &Message) {
    if is_own_message(&ctx, &msg) {
        return;
    }
    let new_content: String = get_new_messages(&msg);
    if new_content.len() > 0 {
        match msg.reply(&ctx.http, new_content).await {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}

fn get_new_messages(msg: &Message) -> String {
    let mut new_content: Vec<String> = vec![];
    let replaced_new_lines = msg.content.replace("\n", " ");
    let words = replaced_new_lines.split_whitespace();
    for word in words {
        let matcher = ["/x.com", "/twitter.com"];
        for twitter_uri in matcher.iter() {
            if word.contains(twitter_uri) {
                let new_url = word.replace(twitter_uri, "/vxtwitter.com");
                if !new_content.contains(&new_url) {
                    new_content.push(new_url)
                }
            }
        }
    }

    new_content.join(", ")
}
