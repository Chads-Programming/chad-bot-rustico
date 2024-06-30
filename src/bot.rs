use serenity::all::{Message, MessageBuilder, ResolvedOption, ResolvedValue};
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

impl Handler {
    pub fn new(guild_id: u64) -> Self {
        Self { guild_id }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content = match command.data.name.as_str() {
                "say_hello" => Some(commands::say_hello::run(&command.data.options())),
                "fox" => Some(commands::fox::run(&command.user, &command.data.options())),
                "bans_info" => Some(commands::bans_info::run(&ctx, &command).await),
                "warn" => {
                    let user_option = utils::get_user_from_query(&command.data.options());

                    match user_option {
                        Some(user) => Some(commands::warn::run(&ctx, &user).await),
                        None => Some("Debe establecer un usuario".to_string()),
                    }
                }
                _ => Some("not implemented :(".to_string()),
            };

            if let Some(content) = content {
                let data = CreateInteractionResponseMessage::new().content(content);
                let builder: CreateInteractionResponse = CreateInteractionResponse::Message(data);
                if let Err(why) = command.create_response(&ctx.http, builder).await {
                    info!("Cannot respond to slash command: {}", why);
                }
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let channel = match msg.channel_id.to_channel(&ctx).await {
            Ok(channel) => channel,
            Err(why) => {
                println!("Error getting channel: {why:?}");

                return;
            }
        };

        let response = MessageBuilder::new()
            .push("User")
            .push_bold_safe(&msg.author.name)
            .push(" used the 'ping' command in the ")
            .mention(&channel)
            .push(" channel")
            .build();

        if let Err(why) = msg.channel_id.say(&ctx.http, &response).await {
            println!("Error sending message: {why:?}");
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        // println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(self.guild_id);

        let commands = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::say_hello::register(),
                    commands::fox::register(),
                    commands::bans_info::register(),
                    commands::warn::register(),
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
