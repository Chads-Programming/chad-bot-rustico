use serenity::all::CommandInteraction;
use serenity::async_trait;
use serenity::{
    all::{
        Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
        GatewayIntents, GuildId, Interaction, Ready,
    },
    Client,
};

use tracing::log::info;

use crate::{commands, utils};

struct Handler {
    pub guild_id: u64,
}

struct ContentPayload {
    content: Option<String>,
    ephemeral: bool,
}

impl ContentPayload {
    pub fn simple(content: Option<String>) -> Self {
        Self {
            content,
            ephemeral: false,
        }
    }

    pub fn simple_ephemeral(content: Option<String>) -> Self {
        Self {
            content,
            ephemeral: true,
        }
    }

    pub fn default() -> Self {
        Self {
            content: Some("Not implemented".to_string()),
            ephemeral: false,
        }
    }
}

impl Handler {
    pub fn new(guild_id: u64) -> Self {
        Self { guild_id }
    }

    pub async fn dispatch_response(
        ctx: &Context,
        command: &CommandInteraction,
        content: String,
        ephemeral: bool,
    ) {
        let data = CreateInteractionResponseMessage::new()
            .content(content)
            .ephemeral(ephemeral);

        let builder: CreateInteractionResponse = CreateInteractionResponse::Message(data);

        if let Err(why) = command.create_response(&ctx.http, builder).await {
            info!("Cannot respond to slash command: {}", why);
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content_payload = match command.data.name.as_str() {
                "say_hello" => {
                    ContentPayload::simple(Some(commands::say_hello::run(&command.data.options())))
                }
                "fox" => ContentPayload::simple(Some(commands::fox::run(
                    &command.user,
                    &command.data.options(),
                ))),
                "bans_info" => {
                    ContentPayload::simple(Some(commands::bans_info::run(&ctx, &command).await))
                }
                "propose_project" => {
                    commands::propose_project::run(&ctx, &command)
                        .await
                        .unwrap();
                    ContentPayload::default()
                }
                "list_projects" => {
                    ContentPayload::simple(Some(commands::list_projects::run(&ctx).await.unwrap()))
                }
                "warn" => {
                    let user_option = utils::get_user_from_query(&command.data.options());

                    let content = match user_option {
                        Some(user) => Some(commands::warn::run(&ctx, &user).await),
                        None => Some("Debe establecer un usuario".to_string()),
                    };

                    ContentPayload::simple_ephemeral(content)
                }
                _ => ContentPayload::default(),
            };

            if let Some(content) = content_payload.content {
                Handler::dispatch_response(&ctx, &command, content, content_payload.ephemeral)
                    .await;
            }
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        let guild_id = GuildId::new(self.guild_id);

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::say_hello::register(),
                    commands::fox::register(),
                    commands::bans_info::register(),
                    commands::warn::register(),
                    commands::propose_project::register(),
                    commands::list_projects::register(),
                ],
            )
            .await
            .unwrap();

        info!("Registered commands: {:#?}", commands);
    }
}

pub async fn setup(token: String, guild_id: u64) -> Client {
    // Build our client.
    Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler::new(guild_id))
        .await
        .expect("Err creating client")
}
