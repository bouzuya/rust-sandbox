use entity::{StampRally, User};

#[test]
fn join_test() {
    let stamp_rally = StampRally::new();
    let user = User::new();

    let player = stamp_rally.join(user.id());
    assert_eq!(player.stamp_rally_id(), stamp_rally.id());
    assert_eq!(player.user_id(), user.id());
}

#[test]
fn issue_test() -> anyhow::Result<()> {
    let mut stamp_rally = StampRally::new();
    let user = User::new();
    let player = stamp_rally.join(user.id());

    let stamp_card = stamp_rally.issue(player.id())?;

    assert_eq!(stamp_card.player_id(), player.id());
    assert_eq!(stamp_card.stamp_rally_id(), stamp_rally.id());
    assert!(stamp_rally.is_issued(stamp_card.id()));
    Ok(())
}
