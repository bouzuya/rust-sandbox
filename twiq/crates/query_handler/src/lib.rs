pub mod in_memory_user_store;
pub mod update_query_user;
pub mod user;
pub mod user_store;

pub enum Command {
    UpdateQueryUser(self::update_query_user::Command),
}
