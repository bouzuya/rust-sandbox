#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ScheduledTweet {
    pub text: String,
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::ScheduledTweet;

    fn f(s: &'static str) -> ScheduledTweet {
        ScheduledTweet { text: s.to_owned() }
    }

    #[test]
    fn test() {
        let mut queue = VecDeque::new();

        queue.push_back(f("item1"));
        queue.push_back(f("item2"));
        queue.push_back(f("item3"));
        assert_eq!(queue.pop_front(), Some(f("item1")));
        assert_eq!(queue.pop_front(), Some(f("item2")));
        assert_eq!(queue.pop_front(), Some(f("item3")));

        queue.push_back(f("item1"));
        queue.push_back(f("item2"));
        queue.push_back(f("item3"));
        queue.push_back(f("item4"));
        if let Some(removed) = queue.remove(1) {
            queue.insert(2, removed);
        }
        assert_eq!(queue.pop_front(), Some(f("item1")));
        assert_eq!(queue.pop_front(), Some(f("item3")));
        assert_eq!(queue.pop_front(), Some(f("item2")));
        assert_eq!(queue.pop_front(), Some(f("item4")));
    }
}
