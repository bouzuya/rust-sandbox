use crate::config::Config;

#[derive(Clone, Debug, Default)]
pub struct FirestoreUserStore {
    config: Config,
}

impl FirestoreUserStore {
    const QUERY_USER_IDS: &'static str = "query_users";
    const QUERY_TWITTER_USER_IDS: &'static str = "query_twitter_user_ids";
}
