#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    User(crate::aggregate::user::Event),
    UserRequest(crate::aggregate::user_request::Event),
}

// TODO: test serde
