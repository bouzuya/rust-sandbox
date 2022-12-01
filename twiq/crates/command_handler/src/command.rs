pub mod create_user_request;
pub mod request_user;
pub mod send_user_request;
pub mod update_user;

pub enum Command {
    CreateUserRequest(self::create_user_request::Command),
    RequestUser(self::request_user::Command),
    SendUserRequest(self::send_user_request::Command),
    UpdateUser(self::update_user::Command),
}

// async fn handler<C>(context: &C, command: Command) -> ...
