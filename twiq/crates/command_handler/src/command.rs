pub mod request_user;

pub enum Command {
    RequestUser(self::request_user::Command),
}

// async fn handler<C>(context: &C, command: Command) -> ...
