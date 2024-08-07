use super::setup::RouterState;
use crate::consts;
use crate::utils;
use axum::routing::post;
use axum::Router;
use reqwest::StatusCode;

use axum::{extract::State, response::IntoResponse};
use tracing::{error, info};

async fn reminder_good_night(State(ctx): State<RouterState>) -> impl IntoResponse {
    let response = utils::send_message_to_channel(
        &ctx.0,
        consts::GENERAL_CHANNEL_ID,
        "Buenas noches gente".to_string(),
        Some(consts::CAT_CHAD_STICKER),
    )
    .await;

    if let Err(err) = response {
        error!("Error on send message: {err}");

        return (StatusCode::BAD_GATEWAY, "Error on  \"good night\" reminder").into_response();
    }

    info!("Message was sending to channel [GENERAL]");

    (StatusCode::OK, "Published \"good night\" reminder repos").into_response()
}

async fn reminder_good_morning(State(ctx): State<RouterState>) -> impl IntoResponse {
    let response = utils::send_message_to_channel(
        &ctx.0,
        consts::GENERAL_CHANNEL_ID,
        "Buenos días gente".to_string(),
        Some(consts::BASED_CAT_STICKER),
    )
    .await;

    if let Err(err) = response {
        error!("Error on send message: {err}");

        return (
            StatusCode::BAD_GATEWAY,
            "Error on send  \"good morning\" reminder",
        )
            .into_response();
    }

    info!("Message was sending to channel [GENERAL]");

    (StatusCode::OK, "Published \"good morning\" reminder").into_response()
}

async fn reminder_english_day(State(ctx): State<RouterState>) -> impl IntoResponse {
    let response = utils::send_message_to_channel(
        &ctx.0,
        consts::GENERAL_CHANNEL_ID,
        format!(
            "@here\n Today is the **English Day**, those who are interested could interact on <#{}>",
            consts::ENGLISH_CHANNEL_ID
        )
        .to_string(),
        None,
    )
    .await;

    if let Err(err) = response {
        error!("Error on send message: {err}");

        return (StatusCode::BAD_GATEWAY, "Error ").into_response();
    }

    info!("Message was sending to channel [GENERAL]");

    (StatusCode::OK, "Published \"good morning\" reminder").into_response()
}

pub fn build_router() -> Router<RouterState> {
    Router::new()
        .route("/good-night", post(reminder_good_night))
        .route("/good-morning", post(reminder_good_morning))
        .route("/english-day", post(reminder_english_day))
}
