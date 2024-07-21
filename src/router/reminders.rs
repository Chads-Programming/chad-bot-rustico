use std::path::Path;

use super::setup::RouterState;
use crate::consts;
use crate::utils;
use axum::routing::post;
use axum::Router;
use reqwest::StatusCode;

use axum::{extract::State, response::IntoResponse};
use tracing::{error, info};

async fn reminder_good_night(State(ctx): State<RouterState>) -> impl IntoResponse {
    let response = utils::send_file_message_to_channel(
        &ctx.0,
        consts::GENERAL_CHANNEL_ID,
        "Sigan codeando mis chads",
        Path::new(consts::GREETING_NIGHT_IMAGE),
    )
    .await;

    if let Err(err) = response {
        error!("Error on send message: {err}");

        return (StatusCode::BAD_GATEWAY, "Error on publish repositories").into_response();
    }

    info!("Message was sending to channel [GENERAL]");

    (StatusCode::OK, "Published trending repos").into_response()
}

async fn reminder_good_morning(State(ctx): State<RouterState>) -> impl IntoResponse {
    let response = utils::send_file_message_to_channel(
        &ctx.0,
        consts::GENERAL_CHANNEL_ID,
        "Hora de programar mis chads",
        Path::new(consts::GREETING_DAY_IMAGE),
    )
    .await;

    if let Err(err) = response {
        error!("Error on send message: {err}");

        return (StatusCode::BAD_GATEWAY, "Error on publish repositories").into_response();
    }

    info!("Message was sending to channel [GENERAL]");

    (StatusCode::OK, "Published trending repos").into_response()
}

pub fn build_router() -> Router<RouterState> {
    Router::new()
        .route("/good-night", post(reminder_good_night))
        .route("/good-morning", post(reminder_good_morning))
}
