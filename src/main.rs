mod bot;
mod commands;
mod db;
mod errors;
mod gifs;
mod github;
mod projects;
mod state;
mod utils;
mod events;
mod consts;

use std::sync::Arc;

use octorust::{auth::Credentials, Client};
use projects::repository::ProjectRepository;
use shuttle_runtime::SecretStore;
use state::SharedState;

impl serenity::prelude::TypeMapKey for SharedState {
    type Value = SharedState;
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    let token = secret_store.get("DISCORD_TOKEN").expect("token required");
    let guild_id: u64 = secret_store
        .get("DISCORD_GUILD_ID")
        .expect("guild id required")
        .parse()
        .expect("integer required");

    let github_token = secret_store.get("GITHUB_TOKEN").expect("github token required");

    let connection_url = secret_store.get("DATABASE_URL").expect("base url required");
    let client = bot::setup(token, guild_id).await;
    let pool = db::get_pool(&connection_url.clone()).await;

    let github_client = Client::new(
        String::from("user-agent-name"),
        Credentials::Token(github_token),
    )
    .unwrap();

    let mut data = client.data.write().await;

    data.insert::<SharedState>(SharedState {
        project_repository: ProjectRepository::new(Arc::new(pool)),
        github_client,
    });

    drop(data);

    Ok(client.into())
}
