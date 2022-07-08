struct UserNotificationMentionPreferences {
    mentions: bool,
    at_channel: bool,
    at_here: bool,
}

struct UserNotificationPreferences {
    all_new_messages: bool,
    mentions: UserNotificationMentionPreferences,
    nothing: bool,
}

struct Channel {
    muted: bool,
    notification_preferences: UserNotificationPreferences,
}

enum MessageType {
    ChannelMessage,
    DirectMessage,
    ThreadMessage,
}

struct MessageThread {
    user_subscribed: bool,
}

struct Message {
    channel: Option<Channel>,
    thread: Option<MessageThread>,
    direct_message: Option<MessageThread>,
    message: String,
    mentions: Vec<String>,
    message_type: MessageType,
}

enum UserStatus {
    Active,
    DoNotDisturb,
    Inactive,
}

struct UserDevicePreferences {
    notification_preferences: UserNotificationPreferences,
}

struct UserPreferences {
    threads_everything: bool,
    notification_preferences: UserNotificationPreferences,
    device_preferences: UserDevicePreferences,
}

struct User {
    status: UserStatus,
    preferences: UserPreferences,
}

struct Notification {
    dnd_override: bool,
    message: Message,
    user: User,
}

fn is_channel_muted(n: &Notification) -> bool {
    n.message.channel.as_ref().unwrap().muted
}

fn is_thread_message(n: &Notification) -> bool {
    matches!(n.message.message_type, MessageType::ThreadMessage)
}

fn is_user_subscribed(n: &Notification) -> bool {
    n.message.thread.as_ref().unwrap().user_subscribed
}

fn is_user_in_dnd(n: &Notification) -> bool {
    matches!(n.user.status, UserStatus::DoNotDisturb)
}

fn is_dnd_override_on(n: &Notification) -> bool {
    n.dnd_override
}

fn should_slack_send_a_notification(n: &Notification) -> bool {
    if is_channel_muted(&n) {
        if !is_thread_message(&n) || !is_user_subscribed(&n) {
            return false;
        }
    }
    if is_user_in_dnd(&n) && !is_dnd_override_on(&n) {
        return false;
    }
    return true;
}

#[test]
fn test_do_not_send_notification_when_user_in_dnd_and_dnd_override_is_off() {
    let n = Notification {
        dnd_override: false,
        message: Message {
            channel: Some(Channel {
                muted: false,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
            }),
            thread: None,
            direct_message: None,
            message: "Hey there".to_string(),
            mentions: [].to_vec(),
            message_type: MessageType::ChannelMessage,
        },
        user: User {
            status: UserStatus::DoNotDisturb,
            preferences: UserPreferences {
                threads_everything: true,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
                device_preferences: UserDevicePreferences {
                    notification_preferences: UserNotificationPreferences {
                        all_new_messages: true,
                        mentions: UserNotificationMentionPreferences {
                            mentions: false,
                            at_channel: false,
                            at_here: false,
                        },
                        nothing: false,
                    },
                },
            },
        },
    };

    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_do_not_send_notification_when_user_in_dnd_and_dnd_override_is_on() {
    let n = Notification {
        dnd_override: true,
        message: Message {
            channel: Some(Channel {
                muted: false,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
            }),
            thread: None,
            direct_message: None,
            message: "Hey there".to_string(),
            mentions: [].to_vec(),
            message_type: MessageType::ChannelMessage,
        },
        user: User {
            status: UserStatus::DoNotDisturb,
            preferences: UserPreferences {
                threads_everything: true,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
                device_preferences: UserDevicePreferences {
                    notification_preferences: UserNotificationPreferences {
                        all_new_messages: true,
                        mentions: UserNotificationMentionPreferences {
                            mentions: false,
                            at_channel: false,
                            at_here: false,
                        },
                        nothing: false,
                    },
                },
            },
        },
    };

    assert_eq!(should_slack_send_a_notification(&n), true)
}

#[test]
fn test_do_not_send_notification_when_channel_is_muted_and_is_not_thread() {
    let n = Notification {
        dnd_override: true,
        message: Message {
            channel: Some(Channel {
                muted: true,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
            }),
            thread: None,
            direct_message: None,
            message: "Hey there".to_string(),
            mentions: [].to_vec(),
            message_type: MessageType::ChannelMessage,
        },
        user: User {
            status: UserStatus::Active,
            preferences: UserPreferences {
                threads_everything: true,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
                device_preferences: UserDevicePreferences {
                    notification_preferences: UserNotificationPreferences {
                        all_new_messages: true,
                        mentions: UserNotificationMentionPreferences {
                            mentions: false,
                            at_channel: false,
                            at_here: false,
                        },
                        nothing: false,
                    },
                },
            },
        },
    };

    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_do_not_send_notification_when_channel_is_muted_and_is_thread_but_user_isnt_subscribed() {
    let n = Notification {
        dnd_override: true,
        message: Message {
            channel: Some(Channel {
                muted: true,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
            }),
            thread: Some(MessageThread {
                user_subscribed: false,
            }),
            direct_message: None,
            message: "Hey there".to_string(),
            mentions: [].to_vec(),
            message_type: MessageType::ChannelMessage,
        },
        user: User {
            status: UserStatus::Active,
            preferences: UserPreferences {
                threads_everything: true,
                notification_preferences: UserNotificationPreferences {
                    all_new_messages: true,
                    mentions: UserNotificationMentionPreferences {
                        mentions: false,
                        at_channel: false,
                        at_here: false,
                    },
                    nothing: false,
                },
                device_preferences: UserDevicePreferences {
                    notification_preferences: UserNotificationPreferences {
                        all_new_messages: true,
                        mentions: UserNotificationMentionPreferences {
                            mentions: false,
                            at_channel: false,
                            at_here: false,
                        },
                        nothing: false,
                    },
                },
            },
        },
    };

    assert_eq!(should_slack_send_a_notification(&n), false)
}
