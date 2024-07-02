use serenity::all::{
    CommandInteraction, Context, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use serenity::builder::CreateCommand;
use serenity::utils::CreateQuickModal;

use crate::projects::models::CreateProject;
use crate::state::SharedState;

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<(), serenity::Error> {
    let data = ctx.data.read().await;
    let state = &data.get::<SharedState>().unwrap();
    let respository = &state.project_repository;

    let owner_name = interaction.user.name.clone();

    let modal = CreateQuickModal::new("Proponer una idea de proyecto")
        .timeout(std::time::Duration::from_secs(600))
        .short_field("Título")
        .paragraph_field("Descripción");

    let response = interaction.quick_modal(ctx, modal).await?.unwrap();

    let inputs = response.inputs;
    let (title, description) = (&inputs[0], &inputs[1]);

    let creation_response = respository
        .create(&CreateProject {
            title: title.clone(),
            description: description.clone(),
            owner_name,
        })
        .await;

    match creation_response {
        Ok(_) => {
            response
        .interaction
        .create_response(
            ctx,
            CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                format!("**Idea registrada**\n\n**Título**: {title}\n**Descripción**: {description}\n\n🦊 🚬"),
            ).ephemeral(true)),
        )
        .await?;
        }
        Err(_) => {
            response
            .interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().content(
                    "**No se a podido crear el proyecto**".to_string(),
                ).ephemeral(true)),
            )
            .await?;
        }
    }

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("propose_project").description("Proponer una idea de proyecto")
}
