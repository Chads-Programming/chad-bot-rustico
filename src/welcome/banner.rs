use std::path::Path;

use chrono::Utc;
use serenity::{
    all::{Context, GuildId, Member, UserId},
    futures::TryFutureExt,
};

use crate::{consts, utils};
use tracing::log::error as log_error;

pub async fn send_welcome_banner(guild_id: &GuildId, ctx: &Context, member: &Member) {
    let response = reqwest::get(member.face()).await.unwrap();
    let avatar = response.bytes().await.unwrap();

    let position_number = guild_id
        .to_guild_cached(&ctx)
        .map(|g| g.member_count as usize)
        .unwrap_or(1);

    let output_path = format!("/tmp/{}_welcome.png", member.user.name);

    let dt = Utc::now();
    let timestamp: String = dt.timestamp().to_string();

    let digits: Vec<_> = timestamp
        .chars()
        .map(|d| d.to_digit(10).unwrap_or(5))
        .collect();

    let random_number = *digits.last().unwrap_or(&5);

    let banner = if random_number > 3 {
        "./assets/banner.png"
    } else {
        "./assets/pride_banner.png"
    };

    if let Err(err) = gen_image::generate(
        &avatar,
        banner,
        member.distinct(),
        position_number,
        &output_path,
        include_bytes!("../../assets/fonts/Roboto-Bold.ttf"),
        include_bytes!("../../assets/fonts/Roboto-Regular.ttf"),
    ) {
        log_error!("{err:?}");
    }

    let send_result = utils::send_file_message_to_channel(
        &ctx.http,
        consts::WELCOME_CHANNEL_ID,
        &format!("Bienvenido a este humilde servidor: <@{}>", member.user.id),
        Path::new(&output_path),
    )
    .await;

    if let Err(err) = send_result {
        log_error!("{err:?}");
    }
}
