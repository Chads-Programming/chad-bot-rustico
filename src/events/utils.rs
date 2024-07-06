use serenity::all::Context;
use serenity::all::Message;

pub fn is_own_message(ctx: &Context, msg: &Message) -> bool {
    ctx.cache.current_user().id == msg.author.id
}
