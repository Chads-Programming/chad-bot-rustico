use serenity::all::{ChannelId, Http, ResolvedOption, ResolvedValue, User};
use tracing::{error, info};

#[allow(suspicious_double_ref_op)]
pub fn get_user_from_query(options: &[ResolvedOption]) -> Option<User> {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    {
        return Some(user.clone().clone());
    }

    None
}

pub async fn send_message_to_channel(
    http: &Http,
    channel_id: u64,
    message: String,
) -> Result<(), serenity::Error> {
    let channel = ChannelId::new(channel_id);

    if let Err(err) = channel.say(http, message).await {
        error!("Error on send message: {err}");

        return Err(err);
    }

    info!("Message was sending to channel");

    Ok(())
}
