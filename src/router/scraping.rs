use super::setup::RouterState;
use crate::{consts, utils};
use axum::routing::post;
use axum::{extract::State, response::IntoResponse};
use axum::{Json, Router};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serenity::all::{ChannelId, Colour, CreateEmbed};
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

    let embeds = body
        .into_iter()
        .map(|free_course| {
            CreateEmbed::new()
                .title(format!("**{}**", free_course.title))
                .field("**Código**", free_course.code, true)
                .field("**Tiempo para reclamar**", free_course.countdown, true)
                .url(free_course.link)
                .color(Colour::from_rgb(15, 137, 64))
        })
        .collect::<Vec<CreateEmbed>>();

    let courses_embeds_chunks = embeds.chunks(course_chunk_size);

    let mut errors = 0;
    let mut current_chunk = 1;

    for embeds in courses_embeds_chunks.into_iter() {
        let text_content = if current_chunk == 1 {
            Some("@here\n# Cursos gratis de la semana 🦊🚬\n".to_string())
        } else {
            None
        };

        let send_response = utils::send_embeds_to_channel(
            &ctx.0,
            consts::COURSES_CHANNEL_ID,
            embeds.to_vec(),
            text_content,
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
