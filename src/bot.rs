use std::path::Path;

use crate::{commands, consts, events, utils};
use serenity::all::{
    CommandInteraction, CreateAllowedMentions, CreateInteractionResponseFollowup, Member, Message,
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
}

impl From<Result<String, serenity::Error>> for ContentPayload {
    fn from(result: Result<String, serenity::Error>) -> Self {
        match result {
            Ok(msg) => Self::from_str(msg),
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
        let response = reqwest::get(member.face()).await.unwrap();
        let avatar = response.bytes().await.unwrap();
        let guild_id = GuildId::new(self.guild_id);

        let position_number = guild_id
            .to_guild_cached(&ctx)
            .map(|g| g.member_count as usize)
            .unwrap_or(1);

        let output_path = format!("/tmp/{}_welcome.png", member.user.name);

        if let Err(err) = gen_image::generate(
            &avatar,
            "./assets/welcome.png",
            member.distinct(),
            position_number,
            &output_path,
            include_bytes!("../assets/fonts/WorkSans-Bold.ttf"),
            include_bytes!("../assets/fonts/WorkSans-Regular.ttf"),
        ) {
            log_error!("{err:?}");
        }

        if let Err(err) = utils::send_file_message_to_channel(
            &ctx.http,
            consts::WELCOME_CHANNEL_ID,
            "",
            Path::new(&output_path),
        )
        .await
        {
            log_error!("{err:?}");
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let content_payload = match command.data.name.as_str() {
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
                    let user_option = utils::get_user_from_query(&command.data.options());

                    let content = match user_option {
                        Some(user) => commands::warn::run(&ctx, &user).await,
                        None => "Debe establecer un usuario".to_string(),
                    };

                    ContentPayload::from_str(content).ephemeral(true)
                }
                "coders_leaderboard" => commands::coders_leaderboard::run(&ctx).await.into(),
                "wallet_leaderboard" => {
                    if let Err(why) = command.defer(&ctx.http).await {
                        log_error!("Error deferring interaction: {:?}", why);

                        return;
                    }
                    ContentPayload::from_str(commands::wallet_leaderboard::run(&ctx).await)
                        .defer(true)
                }
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
