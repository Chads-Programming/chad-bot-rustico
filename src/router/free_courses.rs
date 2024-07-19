use axum::Json;
use axum::{extract::State, response::IntoResponse};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{consts, utils};

use super::setup::RouterState;

pub type FreeCourses = Vec<FreeCourse>;

#[derive(Serialize, Deserialize)]
pub struct FreeCourse {
    title: String,
    code: String,
    countdown: String,
    link: String,
}

pub async fn publish_free_courses(
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

        let send_response =
            utils::send_message_to_channel(&ctx.0, consts::COURSES_CHANNEL_ID, formated_message)
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
