use event_store_core::Event as RawEvent;

pub enum Event {
    User(crate::aggregate::user::Event),
    UserRequest(crate::aggregate::user_request::Event),
}

impl From<Event> for RawEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::User(e) => RawEvent::from(e),
            Event::UserRequest(e) => RawEvent::from(e),
        }
    }
}

// TODO: test serde
