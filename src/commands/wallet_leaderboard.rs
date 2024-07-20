use serenity::all::{Context, CreateCommand};

use crate::state::SharedState;

pub async fn run(ctx: &Context) -> String {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let wallet_service = &state.wallet_service;

    let mut leaderboard = String::new();

    match wallet_service.find_members_sort_by_wallet_amount().await {
        Ok(members) => {
            let mut count = 0;
            for member in members {
                count += 1;
                let wallet = member.wallet;
                let new_val = format!(
                    "**{}** *<@{}>* `[{}]`\n",
                    count, member.discord_id, wallet.amount
                )
                .as_str()
                .to_owned();
                leaderboard.push_str(&new_val);
            }
        }
        Err(err) => {
            return format!("Error: {}", err);
        }
    }

    format!("\n**Top Richachones:**\n\n {leaderboard} \n\n🤑 🏦 ")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("wallet_leaderboard")
        .description("Mostrar el leaderboard de los usuarios con más dinero en el servidor")
}
