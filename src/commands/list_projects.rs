use serenity::all::Context;
use serenity::builder::CreateCommand;

use crate::state::SharedState;

pub async fn run(ctx: &Context) -> Result<String, serenity::Error> {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let repository = &state.project_repository;

    let find_response = repository.find_all().await;

    match find_response {
        Ok(response) => {
            let list_str = response
                .data
                .into_iter()
                .map(|project| {
                    format!(
                        "**{}**\nby: *{}*\n{}",
                        project.title, project.owner_name, project.description
                    )
                })
                .collect::<Vec<String>>()
                .join("\n\n");

            Ok(format!(
                "**Listado de ideas**\n**total propuestas**: {} \n\n{}\n\n🦊 🚬",
                response.total, list_str
            ))
        }
        Err(_) => Ok("Ha ocurrido un error al listar".to_string()),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("list_projects").description("Listar ideas de projectos")
}
