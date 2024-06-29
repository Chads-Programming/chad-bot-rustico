use serenity::all::User;
use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(user: &User, _options: &[ResolvedOption]) -> String {
    let name = user.name.clone();

    format!("{name} por favor manten a tu familia lejos de fox!")
}

pub fn register() -> CreateCommand {
    CreateCommand::new("fox").description("Dar contexto sobre fox")
}
