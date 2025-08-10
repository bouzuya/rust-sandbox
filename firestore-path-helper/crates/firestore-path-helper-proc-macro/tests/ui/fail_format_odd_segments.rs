fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!(
        "users/{user_id}/repos",
        user_id = String
    );
}
