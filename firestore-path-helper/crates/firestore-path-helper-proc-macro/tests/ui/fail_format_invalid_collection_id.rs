fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!("user$/{user_id}", user_id = String);
}
