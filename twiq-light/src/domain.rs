#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct MyTweet {
    pub id_str: String,
    pub at: String,
    pub text: String,
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    #[test]
    fn test() {
        let mut queue = VecDeque::new();

        queue.push_back("item1");
        queue.push_back("item2");
        queue.push_back("item3");

        assert_eq!(
            queue
                .iter()
                .copied()
                .collect::<Vec<&'static str>>()
                .join("\n"),
            "item1\nitem2\nitem3\n"
        );

        assert_eq!(queue.pop_front(), Some("item1"));
        assert_eq!(queue.pop_front(), Some("item2"));
        assert_eq!(queue.pop_front(), Some("item3"));
    }
}
