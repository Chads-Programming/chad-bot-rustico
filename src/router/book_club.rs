use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use reqwest::StatusCode;
use serde::Deserialize;
use tracing::error;

use crate::{book, consts, utils};

use super::setup::RouterState;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookSubmission {
    pub title: String,
    pub authors: Vec<String>,
    pub description: String,
    pub cover_url: String,
    pub votes: i32,
    pub creator_id: String,
}

async fn publish_book_winner(
    State(ctx): State<RouterState>,
    Json(body): Json<BookSubmission>,
) -> impl IntoResponse {
    let embeb_book_result = book::create_book_embeb(body).await;

    if embeb_book_result.is_err() {
        return (StatusCode::BAD_GATEWAY, "Error on embed book").into_response();
    }

    let response = utils::send_embeds_to_channel(
        &ctx.0,
        consts::NOTIFICATIONS_CHANNEL_ID,
        vec![embeb_book_result.unwrap()],
        Some("Libro elegido por el club".to_string()),
    )
    .await;

    if let Err(err) = response {
        error!("Error on send message: {err}");

        return (
            StatusCode::BAD_GATEWAY,
            "Error on  \"publish book\" reminder",
        )
            .into_response();
    }

    tracing::info!("Message was sending to channel [COMUNICADOS]");

    (StatusCode::OK, "Published \"winner book\" reminder").into_response()
}

pub fn build_router() -> Router<RouterState> {
    Router::new().route("/publish-winner-book", post(publish_book_winner))
}
