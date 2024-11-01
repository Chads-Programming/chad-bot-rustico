use std::path::Path;

use serenity::all::{ChannelId, CreateAttachment, CreateEmbed, CreateMessage, Http, StickerId};
use tracing::{error, info};

pub async fn send_message_to_channel(
    http: &Http,
    channel_id: u64,
    message: String,
    sticker_id: Option<u64>,
) -> Result<(), serenity::Error> {
    let channel = ChannelId::new(channel_id);

    let mut message = CreateMessage::new().content(message);

    if let Some(sticker) = sticker_id {
        message = message.sticker_id(StickerId::new(sticker));
    }

    if let Err(err) = channel.send_message(http, message).await {
        error!("Error on send message: {err}");

        return Err(err);
    }

    info!("Message was sending to channel");

    Ok(())
}

pub async fn send_embeds_to_channel(
    http: &Http,
    channel_id: u64,
    embeds: Vec<CreateEmbed>,
    content: Option<String>,
) -> Result<(), serenity::Error> {
    let channel = ChannelId::new(channel_id);

    let mut message = CreateMessage::new();

    if let Some(txt) = content {
        message = message.content(txt);
    }

    message = message.add_embeds(embeds);

    if let Err(err) = channel.send_message(http, message).await {
        error!("Error on send message: {err}");

        return Err(err);
    }

    info!("Message was sending to channel");

    Ok(())
}

pub async fn send_file_message_to_channel(
    http: &Http,
    channel_id: u64,
    message: &str,
    file_path: &Path,
) -> Result<(), serenity::Error> {
    let channel = ChannelId::new(channel_id);

    let attachment = CreateAttachment::path(file_path).await?;
    let files: Vec<CreateAttachment> = vec![attachment];
    let message = CreateMessage::new().content(message);

    if let Err(err) = channel.send_files(http, files, message).await {
        error!("Error on send message file: {err}");

        return Err(err);
    }

    info!("Message was sending to channel file");

    Ok(())
}
