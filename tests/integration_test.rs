use should_slack_send_a_notification::slack::*;

fn notification_preferences_factory() -> UserNotificationPreferences {
    UserNotificationPreferences {
        all_new_messages: true,
        mentions: UserNotificationMentionPreferences {
            mentions: true,
            at_channel: true,
            at_here: true,
        },
        nothing: false,
    }
}

fn notification_factory(
    dnd_override_override: Option<bool>,
    message_override: Option<Message>,
    user_override: Option<User>,
) -> Notification {
    let channel = Some(Channel {
        muted: false,
        notification_preferences: notification_preferences_factory(),
    });

    let message = Message {
        channel,
        thread: None,
        direct_message: None,
        message: "Hey there".to_string(),
        mentions: [].to_vec(),
        message_type: MessageType::ChannelMessage,
    };

    let user = User {
        status: UserStatus::Active,
        preferences: UserPreferences {
            threads_everything: true,
            notification_preferences: notification_preferences_factory(),
            device_preferences: UserDevicePreferences {
                notification_preferences: notification_preferences_factory(),
            },
        },
    };

    Notification {
        dnd_override: dnd_override_override.unwrap_or(false),
        message: message_override.unwrap_or(message),
        user: user_override.unwrap_or(user),
    }
}

#[test]
fn test_true_when_thread_message_user_is_subscribed_and_threads_everything_but_channel_mentions_are_not_supressed(
) {
    let channel = Channel {
        muted: false,
        notification_preferences: UserNotificationPreferences {
            all_new_messages: true,
            mentions: UserNotificationMentionPreferences {
                mentions: true,
                at_channel: true,
                at_here: true,
            },
            nothing: false,
        },
    };

    let message = Message {
        channel: Some(channel),
        thread: Some(MessageThread {
            user_subscribed: true,
        }),
        direct_message: None,
        message: "Hey there".to_string(),
        mentions: ["@channel".to_string()].to_vec(),
        message_type: MessageType::ThreadMessage,
    };

    let user = User {
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
    };

    let n = notification_factory(None, Some(message), Some(user));
    assert_eq!(should_slack_send_a_notification(&n), true)
}

#[test]
fn test_false_when_thread_message_user_is_subscribed_and_threads_everything_but_channel_mentions_are_supressed(
) {
    let channel = Channel {
        muted: false,
        notification_preferences: UserNotificationPreferences {
            all_new_messages: false,
            mentions: UserNotificationMentionPreferences {
                mentions: false,
                at_channel: false,
                at_here: false,
            },
            nothing: true,
        },
    };

    let message = Message {
        channel: Some(channel),
        thread: Some(MessageThread {
            user_subscribed: true,
        }),
        direct_message: None,
        message: "Hey there".to_string(),
        mentions: ["@channel".to_string()].to_vec(),
        message_type: MessageType::ThreadMessage,
    };

    let user = User {
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
    };

    let n = notification_factory(None, Some(message), Some(user));
    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_do_not_send_notification_when_is_channel_mention_and_channel_notifications_are_nothing() {
    let channel = Channel {
        muted: false,
        notification_preferences: UserNotificationPreferences {
            all_new_messages: false,
            mentions: UserNotificationMentionPreferences {
                mentions: false,
                at_channel: false,
                at_here: false,
            },
            nothing: true,
        },
    };

    let message = Message {
        channel: Some(channel),
        thread: None,
        direct_message: None,
        message: "Hey there".to_string(),
        mentions: ["@channel".to_string()].to_vec(),
        message_type: MessageType::ChannelMessage,
    };

    let n = notification_factory(Some(false), Some(message), None);
    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_do_not_send_notification_when_is_here_mention_and_channel_notifications_are_at_here() {
    let message = Message {
        channel: Some(Channel {
            muted: false,
            notification_preferences: UserNotificationPreferences {
                all_new_messages: false,
                mentions: UserNotificationMentionPreferences {
                    mentions: false,
                    at_channel: false,
                    at_here: true,
                },
                nothing: false,
            },
        }),
        thread: None,
        direct_message: None,
        message: "Hey there".to_string(),
        mentions: ["@here".to_string()].to_vec(),
        message_type: MessageType::ChannelMessage,
    };

    let n = notification_factory(None, Some(message), None);
    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_do_not_send_notification_when_is_channel_mention_and_channel_notifications_are_at_channel()
{
    let message = Message {
        channel: Some(Channel {
            muted: false,
            notification_preferences: UserNotificationPreferences {
                all_new_messages: false,
                mentions: UserNotificationMentionPreferences {
                    mentions: false,
                    at_channel: true,
                    at_here: false,
                },
                nothing: false,
            },
        }),
        thread: None,
        direct_message: None,
        message: "Hey there".to_string(),
        mentions: ["@channel".to_string()].to_vec(),
        message_type: MessageType::ChannelMessage,
    };

    let n = notification_factory(None, Some(message), None);
    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_send_notification_when_is_channel_mention_and_channel_notifications_are_mentions() {
    let message = Message {
        channel: Some(Channel {
            muted: false,
            notification_preferences: UserNotificationPreferences {
                all_new_messages: false,
                mentions: UserNotificationMentionPreferences {
                    mentions: true,
                    at_channel: false,
                    at_here: false,
                },
                nothing: false,
            },
        }),
        thread: None,
        direct_message: None,
        message: "Hey there".to_string(),
        mentions: ["@channel".to_string()].to_vec(),
        message_type: MessageType::ChannelMessage,
    };

    let n = notification_factory(None, Some(message), None);
    assert_eq!(should_slack_send_a_notification(&n), true)
}

#[test]
fn test_send_notification_when_is_channel_mention_and_channel_notifications_are_all_new_messages() {
    let message = Message {
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
        mentions: ["@channel".to_string()].to_vec(),
        message_type: MessageType::ChannelMessage,
    };

    let n = notification_factory(None, Some(message), None);
    assert_eq!(should_slack_send_a_notification(&n), true)
}

#[test]
fn test_do_not_send_notification_when_user_in_dnd_and_dnd_override_is_off() {
    let user = User {
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
    };

    let n = notification_factory(None, None, Some(user));
    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_do_not_send_notification_when_user_in_dnd_and_dnd_override_is_on() {
    let user = User {
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
    };

    let n = notification_factory(Some(true), None, Some(user));
    assert_eq!(should_slack_send_a_notification(&n), true)
}

#[test]
fn test_do_not_send_notification_when_channel_is_muted_and_is_not_thread() {
    let message = Message {
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
    };

    let n = notification_factory(None, Some(message), None);
    assert_eq!(should_slack_send_a_notification(&n), false)
}

#[test]
fn test_do_not_send_notification_when_channel_is_muted_and_is_thread_but_user_isnt_subscribed() {
    let message = Message {
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
    };

    let n = notification_factory(None, Some(message), None);
    assert_eq!(should_slack_send_a_notification(&n), false)
}
