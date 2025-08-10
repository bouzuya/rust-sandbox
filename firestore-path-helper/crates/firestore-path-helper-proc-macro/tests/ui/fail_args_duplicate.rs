fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!(
        "users/{user_id}",
        user_id = String,
        user_id = String, // duplicate argument
    );
}
