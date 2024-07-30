use super::setup::RouterState;
use crate::{consts, utils};
use axum::routing::post;
use axum::{extract::State, response::IntoResponse};
use axum::{Json, Router};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serenity::all::ChannelId;
use tracing::{error, info};

pub type FreeCourses = Vec<FreeCourse>;

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct MinimalRepository {
    pub id: i64,
    pub full_name: String,
    pub description: String,
    pub stargazers_count: i64,
    pub url: String,
}

#[derive(Serialize, Clone, Debug, Deserialize)]
pub struct MinimalRepositoryBody {
    pub repositories: Vec<MinimalRepository>,
}

#[derive(Serialize, Deserialize)]
pub struct FreeCourse {
    title: String,
    code: String,
    countdown: String,
    link: String,
}

async fn publish_free_courses(
    State(ctx): State<RouterState>,
    Json(body): Json<FreeCourses>,
) -> impl IntoResponse {
    let course_chunk_size = 5;
    let total_courses = body.len();

    let formated_messages = body
        .into_iter()
        .map(|free_course| {
            format!(
                "**{}**\nCódigo: `{}`\nTiempo para reclamar: `{}`\n🔗 [enlace]({})",
                free_course.title, free_course.code, free_course.countdown, free_course.link,
            )
        })
        .collect::<Vec<String>>();

    let courses = formated_messages.chunks(course_chunk_size);

    let mut errors = 0;
    let mut current_chunk = 1;

    for courses_message in courses {
        let msg = courses_message.join("\n\n");

        let formated_message = if current_chunk == 1 {
            format!("@here\n\n**Cursos gratis de la semana**\n\n{msg}")
        } else {
            format!("_\n\n{msg}")
        };

        let send_response = utils::send_message_to_channel(
            &ctx.0,
            consts::COURSES_CHANNEL_ID,
            formated_message,
            None,
        )
        .await;

        if send_response.is_err() {
            errors += 1;
        }

        current_chunk += 1;
    }

    if errors == total_courses {
        return (StatusCode::BAD_GATEWAY, "Error on publish all courses").into_response();
    }

    info!("Message was sending to channel");

    (
        StatusCode::OK,
        format!("Published {total_courses} free courses and failed to send {errors}"),
    )
        .into_response()
}

async fn publish_trending_repos(
    State(ctx): State<RouterState>,
    Json(body): Json<MinimalRepositoryBody>,
) -> impl IntoResponse {
    let channel = ChannelId::new(consts::COMMUNICATIONS_CHANNEL_ID);

    let formated_message = body
        .repositories
        .into_iter()
        .map(|repo| {
            format!(
                "**{}**\n{}\n⭐️`{}` 🔗 [repository link]({})\n",
                repo.full_name, repo.description, repo.stargazers_count, repo.url,
            )
        })
        .collect::<Vec<String>>()
        .join("\n\n");

    match channel
        .say(
            &ctx.0,
            format!(
                "@here\n\n **Repositorios trending de la semana**\n\n {formated_message} \n\n🦊🚬"
            ),
        )
        .await
    {
        Ok(_) => {
            info!("Message was sending to channel");

            (StatusCode::OK, "Published trending repos").into_response()
        }
        Err(err) => {
            error!("Error on send message: {err}");

            (StatusCode::BAD_GATEWAY, "Error on publish repositories").into_response()
        }
    }
}

pub fn build_router() -> Router<RouterState> {
    Router::new()
        .route("/publish-trending-repos", post(publish_trending_repos))
        .route("/publish-free-courses", post(publish_free_courses))
}
