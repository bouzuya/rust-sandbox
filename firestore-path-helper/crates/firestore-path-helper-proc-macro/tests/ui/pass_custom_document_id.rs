fn main() {
    #[derive(Clone, Debug, PartialEq)]
    struct UserId(String);

    impl std::fmt::Display for UserId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl std::str::FromStr for UserId {
        type Err = &'static str;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.is_empty() {
                Err("UserId cannot be empty")
            } else {
                Ok(UserId(s.to_owned()))
            }
        }
    }

    firestore_path_helper_proc_macro::firestore_path_helper!("users/{user_id}", user_id = UserId);

    let user_id = <UserId as std::str::FromStr>::from_str("user123").expect("valid user_id");

    assert_eq!(Collection {}.path(), "users");
    assert_eq!(
        Document {
            user_id: user_id.clone(),
        }
        .path(),
        "users/user123"
    );
    assert_eq!(
        document_id("users/user123").expect("document_id to return Ok"),
        user_id
    );
}
