fn should_slack_send_a_notification(channel_is_muted: bool) -> bool {
    !channel_is_muted
}

#[test]
fn test_channel_is_muted_do_not_send_notification() {
    let channel_is_muted = true;
    assert_eq!(should_slack_send_a_notification(channel_is_muted), false)
}
