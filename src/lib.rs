pub mod slack {
    pub struct UserNotificationMentionPreferences {
        pub mentions: bool,
        pub at_channel: bool,
        pub at_here: bool,
    }

    pub struct UserNotificationPreferences {
        pub all_new_messages: bool,
        pub mentions: UserNotificationMentionPreferences,
        pub nothing: bool,
    }

    pub struct Channel {
        pub muted: bool,
        pub notification_preferences: UserNotificationPreferences,
    }

    pub enum MessageType {
        ChannelMessage,
        DirectMessage,
        ThreadMessage,
    }

    pub struct MessageThread {
        pub user_subscribed: bool,
    }

    pub struct Message {
        pub channel: Option<Channel>,
        pub thread: Option<MessageThread>,
        pub direct_message: Option<MessageThread>,
        pub message: String,
        pub mentions: Vec<String>,
        pub message_type: MessageType,
    }

    pub enum UserStatus {
        Active,
        DoNotDisturb,
        Inactive,
    }

    pub struct UserDevicePreferences {
        pub notification_preferences: UserNotificationPreferences,
    }

    pub struct UserPreferences {
        pub threads_everything: bool,
        pub notification_preferences: UserNotificationPreferences,
        pub device_preferences: UserDevicePreferences,
    }

    pub struct User {
        pub status: UserStatus,
        pub preferences: UserPreferences,
    }

    pub struct Notification {
        pub dnd_override: bool,
        pub message: Message,
        pub user: User,
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

    fn is_channel_mention(n: &Notification) -> bool {
        n.message.mentions.contains(&"@channel".to_string())
    }

    fn is_everyone_mention(n: &Notification) -> bool {
        n.message.mentions.contains(&"@everyone".to_string())
    }

    fn is_here_mention(n: &Notification) -> bool {
        n.message.mentions.contains(&"@here".to_string())
    }

    fn are_channel_mentions_suppressed(n: &Notification) -> bool {
        let np = &n.message.channel.as_ref().unwrap().notification_preferences;

        if np.all_new_messages {
            return false;
        }

        if np.nothing {
            return true;
        }

        if is_channel_mention(&n) && !np.mentions.mentions && !np.mentions.at_channel {
            return true;
        }

        if is_here_mention(&n) && !np.mentions.at_here {
            return true;
        }

        !np.mentions.mentions
    }

    pub fn should_slack_send_a_notification(n: &Notification) -> bool {
        if is_channel_muted(&n) {
            if !is_thread_message(&n) || !is_user_subscribed(&n) {
                return false;
            }
        }
        if is_user_in_dnd(&n) && !is_dnd_override_on(&n) {
            return false;
        }
        if (is_channel_mention(&n) || is_everyone_mention(&n) || is_here_mention(&n))
            && are_channel_mentions_suppressed(&n)
        {
            return false;
        }
        return true;
    }
}
