use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

use serde::{Deserialize, Serialize};
use serenity::all::{CommandInteraction, CreateCommandOption, ResolvedValue};
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use tracing::error;

use crate::errors::CustomError;

#[derive(Serialize, Deserialize, Debug)]
struct CourseInfo {
    name: String,
    link: String,
    description: String,
    authors: Vec<String>,
}

fn read_json_file() -> std::io::Result<String> {
    let mut file = File::open("./assets/data/courses.json")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    Ok(content)
}

fn read_courses_from_json_file() -> Result<HashMap<String, CourseInfo>, CustomError> {
    match read_json_file() {
        Ok(content) => match serde_json::from_str(&content) {
            Ok(json_data) => {
                let mut map: HashMap<String, CourseInfo> = HashMap::new();

                if let serde_json::Value::Object(data) = json_data {
                    for (key, value) in data {
                        let course_info: CourseInfo = serde_json::from_value(value).unwrap();
                        map.insert(key, course_info);
                    }
                }

                Ok(map)
            }
            Err(err) => {
                error!("{err:?}");
                Err(CustomError::FetchError("Error on parsing file".to_string()))
            }
        },
        Err(_) => Err(CustomError::FetchError("Error on reading file".to_string())),
    }
}

pub fn run(interaction: &CommandInteraction) -> Result<String, String> {
    let options = &interaction.data.options().clone();

    let query = {
        let option_course = if let Some(ResolvedOption {
            value: ResolvedValue::String(course),
            ..
        }) = options.first()
        {
            Some(course)
        } else {
            None
        };

        option_course
    };

    let courses = read_courses_from_json_file().unwrap();

    if query.is_none() {
        return Err("No se ha logrado encontrar la información para el curso".to_string());
    }

    if let Some(course) = courses.get(*query.unwrap()) {
        let authors = course
            .authors
            .iter()
            .map(|author| format!("*<@{author}>*"))
            .collect::<Vec<String>>()
            .join(" ");

        return Ok(format!(
            "**{}**\n`{}`\nPor: {}\n[enlace]({})",
            course.name, course.description, authors, course.link
        ));
    }

    Err("No existe el curso".to_string())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("community_courses")
        .description("Consultar los cursos comunitarios que hemos tenido dentro de la comunidad")
        .add_option(
            CreateCommandOption::new(
                serenity::all::CommandOptionType::String,
                "course",
                "El nombre del curso",
            )
            .add_string_choice("DSA", "DSA".to_string())
            .required(true),
        )
}
