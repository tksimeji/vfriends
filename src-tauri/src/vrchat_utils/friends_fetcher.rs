use std::collections::HashSet;

use crate::vrchat_utils::AppResult;
use vrchatapi::apis::configuration::Configuration;
use vrchatapi::apis::friends_api;
use vrchatapi::models;
use crate::auth;

pub async fn fetch_all_friends(state: &auth::AuthState) -> AppResult<Vec<models::LimitedUserFriend>> {
    let config = state.with_session(|session| session.config.clone())?;

    let online_friends = fetch_friend_pages(&config, false).await?;
    let offline_friends = fetch_friend_pages(&config, true).await?;

    Ok(merge_friends_by_id(online_friends, offline_friends))
}

async fn fetch_friend_pages(
    config: &Configuration,
    include_offline: bool,
) -> AppResult<Vec<models::LimitedUserFriend>> {
    const PAGE_SIZE: i32 = 100;

    let mut offset = 0;
    let mut all = Vec::new();

    loop {
        let page =
            friends_api::get_friends(config, Some(offset), Some(PAGE_SIZE), Some(include_offline))
                .await
                .map_err(|err| err.to_string())?;

        let page_size = page.len() as i32;
        if page_size == 0 {
            break;
        }

        all.extend(page);
        if page_size < PAGE_SIZE {
            break;
        }

        offset += PAGE_SIZE;
    }

    Ok(all)
}

fn merge_friends_by_id(
    online_friends: Vec<models::LimitedUserFriend>,
    offline_friends: Vec<models::LimitedUserFriend>,
) -> Vec<models::LimitedUserFriend> {
    let mut seen = HashSet::new();
    online_friends
        .into_iter()
        .chain(offline_friends.into_iter())
        .filter(|friend| seen.insert(friend.id.clone()))
        .collect()
}
