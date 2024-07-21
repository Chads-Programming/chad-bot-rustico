use serenity::all::{Context, CreateCommand};

use crate::state::SharedState;

pub async fn run(ctx: &Context) -> String {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let wallet_service = &state.wallet_service;

    match wallet_service.find_members_sort_by_wallet_amount().await {
        Ok(members) => {
            let leaderboard = members
                .into_iter()
                .enumerate()
                .map(|(index, member)| {
                    format!(
                        "**{}** *<@{}>* `[{}]`\n",
                        index + 1,
                        member.id,
                        member.amount
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n");

            format!("\n**Top Richachones:**\n\n {leaderboard} \n\n🤑 🏦 ")
        }
        Err(err) => {
            format!("Error: {}", err)
        }
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("wallet_leaderboard")
        .description("Mostrar el leaderboard de los usuarios con más dinero en el servidor")
}
