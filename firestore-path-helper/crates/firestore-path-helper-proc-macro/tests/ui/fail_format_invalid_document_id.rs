fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!("users/{u$er_id}", user_id = String);
}
