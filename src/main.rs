mod bot;
mod commands;
mod consts;
mod db;
mod errors;
mod events;
mod gifs;
mod github;
mod projects;
mod state;
mod utils;

use std::sync::Arc;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{routing::get, Router, ServiceExt};
use octorust::{auth::Credentials, Client as GithubClient};
use projects::repository::ProjectRepository;
use serenity::Client as DiscordClient;
use shuttle_runtime::SecretStore;
use state::SharedState;

impl serenity::prelude::TypeMapKey for SharedState {
    type Value = SharedState;
}

pub struct CustomService {
    discord_client: DiscordClient,
    router: Router,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for CustomService {
    async fn bind(mut self, addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        let router = self.router.into_service();

        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

        let serve_router = async move {
            axum::serve(listener, router.into_make_service())
                .await
                .unwrap();
        };

        tokio::select! {
            _ = self.discord_client.start() => {},
            _ = serve_router => {},
        };
        Ok(())
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> Result<CustomService, shuttle_runtime::Error> {
    let token = secret_store.get("DISCORD_TOKEN").expect("token required");
    let guild_id: u64 = secret_store
        .get("DISCORD_GUILD_ID")
        .expect("guild id required")
        .parse()
        .expect("integer required");

    let github_token = secret_store
        .get("GITHUB_TOKEN")
        .expect("github token required");

    let connection_url = secret_store.get("DATABASE_URL").expect("base url required");
    let client = bot::setup(token, guild_id).await;
    let pool = db::get_pool(&connection_url.clone()).await;

    let github_client = GithubClient::new(
        String::from("user-agent-name"),
        Credentials::Token(github_token),
    )
    .unwrap();

    {
        let mut data = client.data.write().await;

        data.insert::<SharedState>(SharedState {
            project_repository: ProjectRepository::new(Arc::new(pool)),
            github_client,
        });
    }

    let router = build_router();

    Ok(CustomService {
        discord_client: client,
        router
    })
}


pub fn build_router() -> Router {
    Router::new().route("/", get(hello_world))
}

pub async fn hello_world() -> impl IntoResponse {
    (StatusCode::OK, "Hello worldss!").into_response()
}