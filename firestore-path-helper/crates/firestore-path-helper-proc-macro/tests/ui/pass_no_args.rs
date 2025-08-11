fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!("users/user1");

    assert_eq!(Collection {}.path(), "users");
    assert_eq!(Document {}.path(), "users/user1");
}
