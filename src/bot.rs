use crate::{commands, consts, events, welcome};
use serenity::all::{
    CommandInteraction, CreateAllowedMentions, CreateEmbed, CreateInteractionResponseFollowup,
    Member, Message,
};
use serenity::async_trait;
use serenity::{
    all::{
        Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
        GatewayIntents, GuildId, Interaction, Ready,
    },
    Client,
};
use tracing::log::{error as log_error, info};

struct Handler {
    pub guild_id: u64,
}

struct EmbedPayload {
    embeds: Vec<CreateEmbed>,
    defer: bool,
}

impl EmbedPayload {
    pub fn new(embeds: Vec<CreateEmbed>) -> Self {
        Self {
            embeds,
            defer: false,
        }
    }

    pub fn defer(mut self, defer: bool) -> Self {
        self.defer = defer;

        self
    }
}

struct ContentPayload {
    content: Option<String>,
    ephemeral: bool,
    defer: bool,
    allowed_mentions: bool,
}

impl ContentPayload {
    pub fn from_str(content: String) -> Self {
        Self {
            content: Some(content),
            ephemeral: false,
            defer: false,
            allowed_mentions: true,
        }
    }

    pub fn ephemeral(mut self, ephemeral: bool) -> Self {
        self.ephemeral = ephemeral;

        self
    }

    pub fn defer(mut self, defer: bool) -> Self {
        self.defer = defer;

        self
    }

    pub fn default() -> Self {
        Self {
            content: Some("Not implemented".to_string()),
            ephemeral: false,
            defer: false,
            allowed_mentions: true,
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
        defer: bool,
        allowed_mentions: bool,
    ) {
        if defer {
            let mut builder = CreateInteractionResponseFollowup::new()
                .content(content)
                .ephemeral(ephemeral);
            if !allowed_mentions {
                builder = builder.allowed_mentions(CreateAllowedMentions::new().empty_users());
            }
            if let Err(why) = command.create_followup(&ctx.http, builder).await {
                info!("Cannot respond to slash command: {}", why);
            }

            return;
        }

        let data = CreateInteractionResponseMessage::new()
            .content(content)
            .ephemeral(ephemeral);

        let builder: CreateInteractionResponse = CreateInteractionResponse::Message(data);

        if let Err(why) = command.create_response(&ctx.http, builder).await {
            info!("Cannot respond to slash command: {}", why);
        }
    }

    pub async fn dispatch_embed_response(
        ctx: &Context,
        command: &CommandInteraction,
        embeds: Vec<CreateEmbed>,
        defer: bool,
    ) {
        if defer {
            let builder = CreateInteractionResponseFollowup::new().embeds(embeds);

            if let Err(why) = command.create_followup(&ctx.http, builder).await {
                info!("Cannot respond to slash command: {}", why);
            }

            return;
        }

        let data = CreateInteractionResponseMessage::new().embeds(embeds);

        let builder: CreateInteractionResponse = CreateInteractionResponse::Message(data);

        if let Err(why) = command.create_response(&ctx.http, builder).await {
            info!("Cannot respond to slash command: {}", why);
        }
    }
}

impl From<Result<String, serenity::Error>> for ContentPayload {
    fn from(result: Result<String, serenity::Error>) -> Self {
        match result {
            Ok(msg) => ContentPayload {
                content: Some(msg),
                ephemeral: false,
                defer: false,
                allowed_mentions: true,
            },
            Err(_) => Self::default(),
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if consts::BOTS_IDS.contains(&msg.author.id.into()) {
            return;
        }

        events::english_day::handle(&ctx, &msg).await;
        events::twitter_links_message::handle(&ctx, &msg).await;
    }

    async fn guild_member_addition(&self, ctx: Context, member: Member) {
        let guild_id = GuildId::new(self.guild_id);

        welcome::banner::send_welcome_banner(&guild_id, &ctx, &member).await;
        welcome::info::send_dm_welcome_information(&ctx, &member.user).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let command_name = command.data.name.as_str();

            let embed_payload = match command_name {
                "wallet_leaderboard" => {
                    if let Err(why) = command.defer(&ctx.http).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    }

                    match commands::wallet_leaderboard::run(&ctx).await {
                        Ok(embed) => Some(EmbedPayload::new(embed).defer(true)),
                        Err(err) => {
                            log_error!("Error deferring interaction: {:?}", err);

                            None
                        }
                    }
                }
                "crypto_prices" => {
                    if let Err(why) = command.defer(&ctx.http).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    }

                    match commands::crypto_prices::run(&ctx, &command).await {
                        Ok(embed) => Some(EmbedPayload::new(vec![embed]).defer(true)),
                        Err(err) => {
                            log_error!("Error on interaction: {:?}", err);

                            None
                        }
                    }
                }
                "meme" => {
                    if let Err(why) = command.defer(&ctx.http).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    }
                    match commands::meme::run(&ctx, &command).await {
                        Ok(embed) => Some(EmbedPayload::new(vec![embed]).defer(true)),
                        Err(err) => {
                            log_error!("Error on interaction: {:?}", err);

                            None
                        }
                    }
                }
                _ => None,
            };

            if let Some(payload) = embed_payload {
                return Handler::dispatch_embed_response(
                    &ctx,
                    &command,
                    payload.embeds,
                    payload.defer,
                )
                .await;
            }

            let content_payload = match command_name {
                "register_wallet" => {
                    if let Err(why) = command.defer_ephemeral(&ctx.http).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    }

                    ContentPayload::from_str(commands::register_wallet::run(&ctx, &command).await)
                        .defer(true)
                }
                "wallet_info" => {
                    if let Err(why) = command.defer_ephemeral(&ctx.http).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    }
                    ContentPayload::from_str(commands::wallet_info::run(&ctx, &command).await)
                        .defer(true)
                }
                "donate_coins" => {
                    if let Err(why) = command.defer(&ctx.http).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    };

                    match commands::donate_coins::run(&ctx, &command).await {
                        Ok(ok_msg) => ContentPayload::from_str(ok_msg).defer(true),
                        Err(err_msg) => ContentPayload::from_str(err_msg).defer(true),
                    }
                }
                "say_hello" => {
                    ContentPayload::from_str(commands::say_hello::run(&command.data.options()))
                }
                "fox" => ContentPayload::from_str(commands::fox::run(
                    &command.user,
                    &command.data.options(),
                )),
                "members_count" => commands::members_count::run(&ctx, &command).await.into(),
                "bans_info" => {
                    ContentPayload::from_str(commands::bans_info::run(&ctx, &command).await)
                        .ephemeral(true)
                }
                "propose_project" => {
                    commands::propose_project::run(&ctx, &command)
                        .await
                        .unwrap();
                    ContentPayload::default()
                }
                "list_projects" => commands::list_projects::run(&ctx).await.into(),
                "warn" => {
                    let content = commands::warn::run(&ctx, &command).await;

                    ContentPayload::from_str(content).ephemeral(true)
                }
                "coders_leaderboard" => {
                    if let Err(why) = commands::coders_leaderboard::run(&ctx, &command).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    }

                    ContentPayload::default()
                }
                "community_courses" => match commands::courses::run(&command) {
                    Ok(courses) => ContentPayload::from_str(courses),
                    Err(err) => ContentPayload::from_str(err).ephemeral(true),
                },
                _ => ContentPayload::default(),
            };

            if let Some(content) = content_payload.content {
                Handler::dispatch_response(
                    &ctx,
                    &command,
                    content,
                    content_payload.ephemeral,
                    content_payload.defer,
                    content_payload.allowed_mentions,
                )
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
                    commands::members_count::register(),
                    commands::bans_info::register(),
                    commands::warn::register(),
                    commands::propose_project::register(),
                    commands::list_projects::register(),
                    commands::coders_leaderboard::register(),
                    commands::register_wallet::register(),
                    commands::donate_coins::register(),
                    commands::wallet_info::register(),
                    commands::wallet_leaderboard::register(),
                    commands::courses::register(),
                    commands::crypto_prices::register(),
                    commands::meme::register(),
                ],
            )
            .await
            .unwrap();

        info!("Registered commands: {:#?}", commands);
    }
}

pub async fn setup(token: String, guild_id: u64) -> Client {
    let intents = GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MEMBERS;
    // Build our client.
    Client::builder(token, intents)
        .event_handler(Handler::new(guild_id))
        .await
        .expect("Err creating client")
}
