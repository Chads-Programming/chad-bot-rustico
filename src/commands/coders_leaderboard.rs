use octorust::types::Contributor;
use serenity::{
    all::{
        CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse,
        CreateInteractionResponseMessage,
    },
    futures::future::join_all,
};
use tracing::error;

use std::collections::HashMap;

use crate::{consts, github, state::SharedState};

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<(), serenity::Error> {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let github_client = &state.github_client;

    let repositories =
        github::repository::find_org_repositories(github_client, consts::GITHUB_ORG_ID).await;

    let repository_names = repositories
        .into_iter()
        .map(|repo_info| repo_info.name)
        .collect::<Vec<String>>();

    let mut future_repo_contributions = vec![];

    for repo_name in repository_names.into_iter() {
        let collaborator = github::repository::find_org_repo_contributors(
            github_client,
            consts::GITHUB_ORG_NAME,
            repo_name,
        );

        future_repo_contributions.push(collaborator);
    }

    let all_contributors = join_all(future_repo_contributions)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<Contributor>>();

    let mut contributor_map: HashMap<String, i64> = HashMap::new();

    for contributor in all_contributors.into_iter() {
        let contributor_name = {
            let login_name = contributor.login.clone();

            if !login_name.is_empty() {
                login_name
            } else {
                contributor.name.clone()
            }
        };

        let current_contributions = contributor_map.get(&contributor_name).unwrap_or(&0);
        let updated_contributions = contributor.contributions + current_contributions;

        contributor_map.insert(contributor_name, updated_contributions);
    }

    let mut reduced_contributors = contributor_map.into_iter().collect::<Vec<(String, i64)>>();
    reduced_contributors.sort_by(|a, b| b.1.cmp(&a.1));

    let contributors_size: usize = reduced_contributors.len();
    let positions = (1..=contributors_size)
        .map(|pos| pos.to_string())
        .collect::<Vec<String>>()
        .join("\n\n");

    let contributors_name = reduced_contributors
        .iter()
        .map(|contrib| contrib.0.to_string())
        .collect::<Vec<String>>()
        .join("\n\n");

    let contributors_total = reduced_contributors
        .into_iter()
        .map(|contrib| contrib.1.to_string())
        .collect::<Vec<String>>()
        .join("\n\n");

    let embed = CreateEmbed::default()
        .title("**Top de contribuidores en el github de la comunidad**  🦊🚬\n")
        .field("#", format!("\n{positions}"), true)
        .field("Github username", format!("\n{contributors_name}"), true)
        .field("Contributions", format!("\n{contributors_total}"), true);

    let msg = CreateInteractionResponseMessage::new().embed(embed);
    let builder: CreateInteractionResponse = CreateInteractionResponse::Message(msg);

    if let Err(why) = command.create_response(&ctx.http, builder).await {
        error!("Cannot respond to slash command: {}", why);
    };

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("coders_leaderboard").description(
        "Mostrar el leaderboard de los contribuidores en los respositorios de la organizacion",
    )
}
