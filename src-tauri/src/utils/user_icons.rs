use vrchatapi::models;

pub fn resolve_user_icon_url(user: &models::User) -> Option<String> {
    let not_empty: fn(&str) -> Option<&str> = |s: &str| (!s.is_empty()).then_some(s);
    not_empty(&user.profile_pic_override)
        .or_else(|| not_empty(&user.user_icon))
        .or_else(|| not_empty(&user.current_avatar_image_url))
        .or_else(|| not_empty(&user.current_avatar_thumbnail_image_url))
        .map(|url| url.to_string())
}
