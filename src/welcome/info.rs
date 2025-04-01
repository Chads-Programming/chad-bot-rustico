use serenity::all::{Context, User, UserId};
use tracing::error;

use crate::consts;

pub async fn send_dm_welcome_information(ctx: &Context, user: &User) {
    match user.create_dm_channel(&ctx.http).await {
        Ok(channel) => {
            channel
                .say(&ctx.http, generate_welcome_information(user.id))
                .await
                .unwrap();
        }
        Err(err) => {
            error!("{err:?}");
        }
    }
}

fn generate_welcome_information(user_id: UserId) -> String {
    let github_information = format!(
        "🌟 **Proyectos de la comunidad** 🌟\n\
        Tu participación es clave para el éxito de nuestra comunidad. Si quieres colaborar y formar parte de nuestros proyectos, puedes colaborar en nuestros [repositorios]({}).\n\
        Aquí podrás contribuir con tus habilidades, ideas y participar activamente en el crecimiento de nuestra comunidad, así podrás desbloquear el badge de `colaborador`.\n", consts::GITHUB_ORGANIZATION
    );

    let wallet_information = String::from(
        "🔗 **Wallet**:\n\
        Aquí te dejamos información importante sobre cómo interactuar con la wallet interna del servidor a través de comandos.\n\n\
        * `/register_wallet`: ¡Registra tu wallet personal!\n\
        * `/donate_coins`: Si alguien del servidor te ha ayudado en algo, puedes donarle **chad-coins** para mostrarle tu gratitud. ¡Cada contribución es muy apreciada!\n\
        * `/wallet_info`: Para ver el estado de tu wallet\n\
        * `/wallet_leaderboard`: ¡Descubre quién lidera la lista de wallets con más dinero! ¿Serás tú el próximo en llegar a la cima?\n\n\
        Recuerda registrarte primero en la wallet y cada semana se te acreditarán **chad-coins** (solo válido dentro del servidor) ¡Estamos emocionados de que formes parte!"
    );

    let website = "Visita nuestro website: https://chads-programming.dev";

    format!(
        "🎉 **Bienvenido/a: ** <@{}> 🎉\nTe dejamos presente la siguiente información: \n\n{github_information}\n\n{wallet_information}\n\n{website}",
        user_id
    )
}
