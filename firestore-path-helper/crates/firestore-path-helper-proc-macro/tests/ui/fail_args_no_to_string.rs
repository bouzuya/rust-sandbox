fn main() {
    struct RepoId(String);

    // RepoId does not implement ToString, so it should cause a compile error.

    firestore_path_helper_proc_macro::firestore_path_helper!(
        "users/{user_id}/repos/{repo_id}/issues/{issue_number}",
        user_id = String,
        repo_id = RepoId,
        issue_number = u32,
    );
}
