fn main() {
    struct RepoId(String);

    impl std::fmt::Display for RepoId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    firestore_path_helper_proc_macro::firestore_path_helper!(
        "users/{user_id}/repos/{repo_id}/issues/{issue_number}",
        user_id = String,
        repo_id = RepoId,
        issue_number = u32,
    );

    assert_eq!(
        Document {
            user_id: String::from("user123"),
            repo_id: RepoId(String::from("repo456")),
            issue_number: 789,
        }
        .path(),
        "users/user123/repos/repo456/issues/789"
    );
}
