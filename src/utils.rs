use serenity::all::{ResolvedOption, ResolvedValue, User};

pub fn get_user_from_query(options: &[ResolvedOption]) -> Option<User> {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _), ..
    }) = options.first()
    {
      return Some(user.clone().clone());
    }

    None
}
