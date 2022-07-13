use should_slack_send_a_notification::slack::*;

#[test]
fn test_do_not_send_notification_when_is_channel_mention_and_channel_notifications_are_nothing() {
    let n = Notification {
        dnd_override: false,
        message: Message {
            channel: Some(Channel {
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
            }),
            thread: None,
            direct_message: None,
            message: "Hey there".to_string(),
            mentions: ["@channel".to_string()].to_vec(),
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
fn test_do_not_send_notification_when_is_here_mention_and_channel_notifications_are_at_here() {
    let n = Notification {
        dnd_override: false,
        message: Message {
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
fn test_do_not_send_notification_when_is_channel_mention_and_channel_notifications_are_at_channel()
{
    let n = Notification {
        dnd_override: false,
        message: Message {
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
fn test_send_notification_when_is_channel_mention_and_channel_notifications_are_mentions() {
    let n = Notification {
        dnd_override: false,
        message: Message {
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

    assert_eq!(should_slack_send_a_notification(&n), true)
}

#[test]
fn test_send_notification_when_is_channel_mention_and_channel_notifications_are_all_new_messages() {
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
            mentions: ["@channel".to_string()].to_vec(),
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

    assert_eq!(should_slack_send_a_notification(&n), true)
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
