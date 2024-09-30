use serenity::all::{Colour, Context, CreateCommand, CreateEmbed};

use crate::{errors::CustomError, state::SharedState};

pub async fn run(ctx: &Context) -> Result<Vec<CreateEmbed>, CustomError> {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let wallet_service = &state.wallet_service;

    match wallet_service.find_members_sort_by_wallet_amount().await {
        Ok(members) => {
            let leaderboard_embeds = members
                .into_iter()
                .enumerate()
                .map(|(index, member)| {
                    let position = index + 1;
                    let position_meta = match position {
                        1 => (
                            format!("🥇 **{}st position**", position),
                            Colour::from_rgb(221, 204, 40),
                        ),
                        2 => (
                            format!("🥈 **{}nd position**", position),
                            Colour::from_rgb(135, 136, 138),
                        ),
                        3 => (
                            format!("🥉 **{}rd position**", position),
                            Colour::from_rgb(170, 129, 17),
                        ),
                        _ => (
                            format!("🎖️ **{}th position**", position),
                            Colour::from_rgb(11, 137, 64),
                        ),
                    };

                    CreateEmbed::new()
                        .title(position_meta.0)
                        .field("Amount", format!("`{}`", member.amount), true)
                        .field("Member", format!("<@{}>", member.discord_id), true)
                        .color(position_meta.1)
                })
                .collect::<Vec<CreateEmbed>>();

            Ok(leaderboard_embeds)
        }
        Err(err) => Err(err),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("wallet_leaderboard")
        .description("Mostrar el leaderboard de los usuarios con más dinero en el servidor")
}
