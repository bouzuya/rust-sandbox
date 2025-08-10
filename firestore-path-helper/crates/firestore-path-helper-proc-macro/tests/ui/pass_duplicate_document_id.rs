fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!("users/{id}/repos/{id}", id = String);
}
