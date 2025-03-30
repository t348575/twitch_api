#![doc(alias = "channel.vip.remove")]
//! a user has vip privileges removed on a specified channel.

use super::*;
/// [`channel.vip.remove`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types/#channelvipremove): a user has vip privileges removed on a specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelVipRemoveV1 {
    /// Get notifications when a VIP is removed in this channel.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
}

impl ChannelVipRemoveV1 {
    /// Get notifications for removed VIPs in this channel
    pub fn new(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
        }
    }
}

impl EventSubscription for ChannelVipRemoveV1 {
    type Payload = ChannelVipRemoveV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelVipRemove;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadVips,
        twitch_oauth2::Scope::ChannelManageVips
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.vip.remove`](ChannelVipRemoveV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelVipRemoveV1Payload {
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,

    /// The user ID of the removed vip.
    pub user_id: types::UserId,
    /// The user login of the removed vip.
    pub user_login: types::UserName,
    /// The display name of the removed vip.
    pub user_name: types::DisplayName,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    use crate::eventsub::{Event, Message};

    let payload = r##"
    {
        "subscription": {
            "id": "7297f7eb-3bf5-461f-8ae6-7cd7781ebce3",
            "status": "enabled",
            "type": "channel.vip.remove",
            "version": "1",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337"
            },
            "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2024-02-23T21:12:33.771005262Z"
        },
        "event": {
            "user_id": "1234",
            "user_login": "mod_user",
            "user_name": "Mod_User",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cooler_user",
            "broadcaster_user_name": "Cooler_User"
        }
    }
    "##;

    let val = Event::parse(payload).unwrap();
    crate::tests::roundtrip(&val);

    let Event::ChannelVipRemoveV1(val) = val else {
        panic!("invalid event type");
    };
    let Message::Notification(notif) = val.message else {
        panic!("invalid settings type");
    };

    assert_eq!(notif.broadcaster_user_id.as_str(), "1337");
    assert_eq!(notif.broadcaster_user_login.as_str(), "cooler_user");
    assert_eq!(notif.broadcaster_user_name.as_str(), "Cooler_User");
    assert_eq!(notif.user_id.as_str(), "1234");
    assert_eq!(notif.user_login.as_str(), "mod_user");
    assert_eq!(notif.user_name.as_str(), "Mod_User");
}
