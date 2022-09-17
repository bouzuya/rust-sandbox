pub mod create_user_request;
pub mod request_user;
pub mod update_user;

pub enum Command {
    CreateUserRequest(self::create_user_request::Command),
    RequestUser(self::request_user::Command),
    UpdateUser(self::update_user::Command),
}
