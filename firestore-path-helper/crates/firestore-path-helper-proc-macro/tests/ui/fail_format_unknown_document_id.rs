fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!(
        "users/{user_id}/repos/{repo_id}",
        user_id = String,
        // Missing repo_id argument
    );
}
