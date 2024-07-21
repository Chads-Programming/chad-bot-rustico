use crate::{consts, utils};
use axum::{extract::State, response::IntoResponse, routing::post, Router};
use reqwest::StatusCode;
use std::path::Path;
use tracing::{error, info};

use super::setup::RouterState;

async fn refill(State(ctx): State<RouterState>) -> impl IntoResponse {
    let wallet_service = ctx.1;

    if let Err(err) = wallet_service
        .refill_wallet_members(consts::WALLET_REFILL_AMOUNT)
        .await
    {
        error!("Error on refill wallets: {err}");

        return (StatusCode::GATEWAY_TIMEOUT, "Error on refill wallets").into_response();
    }

    let response = utils::send_file_message_to_channel(
        &ctx.0,
        consts::GENERAL_CHANNEL_ID,
        format!(
            "Día de paga, acaban de recibir su bono a sus wallets {} chad coins",
            consts::WALLET_REFILL_AMOUNT
        )
        .as_str(),
        Path::new(consts::CAT_IMAGE),
    )
    .await;

    if let Err(err) = response {
        error!("Error on send message: {err}");

        return (StatusCode::BAD_GATEWAY, "Error on refilling wallets").into_response();
    }

    info!("Message was sending to channel [GENERAL]");

    (StatusCode::OK, "Wallet refill was ok").into_response()
}

pub fn build_router() -> Router<RouterState> {
    Router::new().route("/refill", post(refill))
}
