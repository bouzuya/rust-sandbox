use crate::{remove::Remove, set::Set};

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Remove(Remove),
    Set(Set),
}
