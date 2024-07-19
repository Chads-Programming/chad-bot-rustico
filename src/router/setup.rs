use axum::middleware::Next;
use serenity::http::Http;
use std::sync::Arc;

use axum::extract::{Request, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{middleware, Router};

use super::{free_courses, health_check, trending_repos};

#[derive(Clone, Debug)]
pub struct RouterSecrets {
    pub bot_api_key: String,
}

#[derive(Clone, Debug)]
pub struct RouterState(pub Arc<Http>);

async fn api_key_strategy(
    State(secrets): State<RouterSecrets>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let header_key = headers.get("Authorization");
    if header_key
        .as_ref()
        .is_some_and(|key_value| key_value.to_str().unwrap() == secrets.bot_api_key)
    {
        return Ok(next.run(req).await);
    }

    tracing::error!(
        "UNAUTHORIZED: {header_key:?} - Local: {}",
        secrets.bot_api_key
    );

    Err(axum::http::StatusCode::UNAUTHORIZED)
}

pub fn build_router(secrets: RouterSecrets, state: RouterState) -> Router {
    Router::new()
        .route("/hello-private", get(health_check::hello_private))
        .route(
            "/publish-trending-repos",
            post(trending_repos::publish_trending_repos),
        )
        .route(
            "/publish-free-courses",
            post(free_courses::publish_free_courses),
        )
        .layer(middleware::from_fn_with_state(secrets, api_key_strategy))
        .route("/hello-chad", get(health_check::hello_chad))
        .with_state(state)
}
