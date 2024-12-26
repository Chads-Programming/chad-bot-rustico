use cripto_api::coin_gecko::CoinGeckoService;

use crate::{projects::repository::ProjectRepository, wallet::services::WalletService};

pub struct SharedState {
    pub project_repository: ProjectRepository,
    pub github_client: octorust::Client,
    pub wallet_service: WalletService,
    pub coin_service: CoinGeckoService,
}
