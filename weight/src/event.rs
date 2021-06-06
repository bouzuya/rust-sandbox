use crate::set::Set;

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    Set(Set),
}
