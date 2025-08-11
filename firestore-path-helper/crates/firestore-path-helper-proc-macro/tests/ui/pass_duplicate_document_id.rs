fn main() {
    firestore_path_helper_proc_macro::firestore_path_helper!("users/{id}/repos/{id}", id = i32);

    assert_eq!(Collection { id: 1 }.path(), "users/1/repos");
    assert_eq!(Document { id: 1 }.path(), "users/1/repos/1");
}
