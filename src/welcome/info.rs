use serenity::all::{Context, User};
use tracing::error;

use crate::consts;

pub async fn send_dm_welcome_information(ctx: &Context, user: &User) {
    match user.create_dm_channel(&ctx.http).await {
        Ok(channel) => {
            channel
                .say(&ctx.http, generate_welcome_information(user.name.clone()))
                .await
                .unwrap();
        }
        Err(err) => {
            error!("{err:?}");
        }
    }
}

fn generate_welcome_information(user_name: String) -> String {
    let github_information = format!(
        "🌟 **¡Únete a nuestros proyectos comunitarios!** 🌟\n\
        Tu participación es clave para el éxito de nuestra comunidad. Si quieres colaborar y formar parte de nuestros emocionantes proyectos, sigue este [enlace]({}).\n\n\
        Aquí podrás contribuir con tus habilidades, ideas y participar activamente en el crecimiento de nuestra comunidad. ¡Te esperamos!", consts::GITHUB_ORGANIZATION
    );

    let wallet_information = String::from(
        "Aquí te dejamos información importante sobre cómo interactuar con la wallet del servidor.\n\n\
        🔗 **Wallet del servidor**:\n\n\
        🔧 **Comandos disponibles**:\n\
        1. **/register-wallet**: ¡Registra tu wallet personal para participar en nuestras actividades financieras!\n\
        2. **/donate**: Si te sientes generoso, puedes donar al servidor usando este comando. ¡Cada contribución es muy apreciada!\n\
        3. **/wallet-leaderboard**: ¡Descubre quién lidera la lista de wallets con más dinero! ¿Serás tú el próximo en llegar a la cima?\n\n\
        Recuerda registrarte primero para poder disfrutar de todos los beneficios de la comunidad. ¡Estamos emocionados de que formes parte!"
    );

    format!("🎉 **¡Bienvenido/a al servidor! {user_name}** 🎉\n{github_information}\n{wallet_information}")
}
