fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!(
        "users/{user_id}/repos/{repo_id}",
        user_id = String,
        repo_id = String,
    );

    let _ = Document {
        user_id: String::from("user123"),
        repo_id: String::from("repo456"),
    };
}
