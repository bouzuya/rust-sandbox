use entity::{StampRally, User};

#[test]
fn join_test() {
    let stamp_rally = StampRally::new();
    let user = User::new();
    let player = stamp_rally.join(user.id());
    assert_eq!(player.stamp_rally_id(), stamp_rally.id());
    assert_eq!(player.user_id(), user.id());
}
