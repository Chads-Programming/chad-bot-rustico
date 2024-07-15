use axum::Json;
use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serenity::all::ChannelId;
use tracing::{error, info};

use crate::consts;

use super::setup::RouterState;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct MinimalRepository {
    pub id: i64,
    pub full_name: String,
    pub description: String,
    pub stargazers_count: i64,
    pub url: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct MinimalRepositoryBody {
    pub repositories: Vec<MinimalRepository>,
}

pub async fn publish_trending_repos(
    State(ctx): State<RouterState>,
    Json(body): Json<MinimalRepositoryBody>,
) -> impl IntoResponse {
    let channel = ChannelId::new(consts::COMMUNICATIONS_CHANNEL_ID);

    let formated_message = body
        .repositories
        .into_iter()
        .map(|repo| {
            format!(
                "**{}**\n{}\n⭐️`{}` 🔗 [repository link]({})\n",
                repo.full_name, repo.description, repo.stargazers_count, repo.url,
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    match channel
        .say(
            &ctx.0,
            format!(
                "@here\n\n **Repositorios trending de la semana**\n\n {formated_message} \n\n🦊🚬"
            ),
        )
        .await
    {
        Ok(_) => {
            info!("Message was sending to channel");

            (StatusCode::OK, "Published trending repos").into_response()
        }
        Err(err) => {
            error!("Error on send message: {err}");

            (StatusCode::BAD_GATEWAY, "Error on publish repositories").into_response()
        }
    }
}
