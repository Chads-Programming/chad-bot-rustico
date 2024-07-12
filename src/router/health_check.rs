use axum::response::IntoResponse;
use reqwest::StatusCode;

pub async fn hello_chad() -> impl IntoResponse {
    (StatusCode::OK, "Hello 🦊 🚬!").into_response()
}

pub async fn hello_private() -> impl IntoResponse {
    (StatusCode::OK, "Hello 🐧 🚬!").into_response()
}
