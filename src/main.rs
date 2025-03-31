mod bot;
mod commands;
mod consts;
mod db;
mod errors;
mod events;
mod github;
mod helpers;
mod meme;
mod projects;
mod router;
mod state;
mod utils;
mod wallet;
mod welcome;

use std::sync::Arc;

use axum::{Router, ServiceExt};
use cripto_api::coin_gecko::CoinGeckoService;
use octorust::{auth::Credentials, Client as GithubClient};
use projects::repository::ProjectRepository;
use router::setup::{RouterSecrets, RouterState};
use serenity::Client as DiscordClient;
use shuttle_runtime::SecretStore;
use state::SharedState;
use wallet::services::WalletService;

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

    let bot_api_key = secret_store.get("API_KEY").expect("bot api key required");
    let gecko_api_key = secret_store
        .get("COIN_GECKO_API_KEY")
        .expect("coin gecko api key is required");

    let connection_url = secret_store.get("DATABASE_URL").expect("base url required");
    let discord_client = bot::setup(token, guild_id).await;
    let pool = db::get_pool(&connection_url.clone()).await;

    let github_client = GithubClient::new(
        String::from("user-agent-name"),
        Credentials::Token(github_token),
    )
    .unwrap();

    let wallet_service = WalletService::new(Arc::new(pool.clone()));

    {
        let mut data = discord_client.data.write().await;

        data.insert::<SharedState>(SharedState {
            project_repository: ProjectRepository::new(Arc::new(pool.clone())),
            github_client: github_client.clone(),
            wallet_service: wallet_service.clone(),
            coin_service: CoinGeckoService::new(gecko_api_key.as_str()),
        });
    }

    let router = router::setup::build_router(
        RouterSecrets { bot_api_key },
        RouterState(discord_client.http.clone(), wallet_service.clone()),
    );

    Ok(CustomService {
        discord_client,
        router,
    })
}
