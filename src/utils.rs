use serenity::all::{ResolvedOption, ResolvedValue, User};

#[allow(suspicious_double_ref_op)]
pub fn get_user_from_query(options: &[ResolvedOption]) -> Option<User> {
    if let Some(ResolvedOption {
        value: ResolvedValue::User(user, _),
        ..
    }) = options.first()
    {
        return Some(user.clone().clone());
    }

    None
}
