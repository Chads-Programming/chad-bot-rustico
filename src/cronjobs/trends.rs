use octorust::Client as GithubClient;
use serenity::all::{CacheHttp, ChannelId};
use tracing::error;

use crate::{consts, github};

pub async fn send_trends(git_client: &GithubClient, http: impl CacheHttp) {
    let response = github::search::trending_repositories(
        git_client,
        &github::search::SearchRepositoryQuery::new(
            Some(100),
            ["nextjs", "typescript", "nestjs"].to_vec(),
        ),
        5,
    )
    .await;

    match response {
        Ok(repos) => {
            let channel = ChannelId::new(consts::COMMUNICATIONS_CHANNEL_ID);

            let formated_message = repos
                .into_iter()
                .map(|repo| {
                    format!(
                        "**{}**\n{}\n⭐️`{}` 🔗 [repository link]({})\n",
                        repo.name, repo.description, repo.stargazers_count, repo.url,
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n");

            match channel
                .say(
                    &http,
                    format!(
                        "@here\n\n **Repositorios de la semana**\n\n {formated_message} \n\n🦊🚬"
                    ),
                )
                .await
            {
                Ok(_) => {
                    println!("Message was sending to channel");
                }
                Err(err) => {
                    error!("Error on send message: {err}");
                }
            }
        }
        Err(_) => {
            error!("Error on fetch trending repos");
        }
    }
}
