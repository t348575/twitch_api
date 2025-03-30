#![doc(alias = "channel.channel_points_custom_reward.remove")]
//! A custom channel points reward has been removed from the specified channel.

use super::*;
/// [`channel.channel_points_custom_reward.remove`](https://dev.twitch.tv/docs/eventsub/eventsub-subscription-types#channelchannel_points_custom_rewardremove): a custom channel points reward has been removed from the specified channel.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPointsCustomRewardRemoveV1 {
    /// The broadcaster user ID for the channel you want to receive channel points custom reward remove notifications for.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    pub broadcaster_user_id: types::UserId,
    /// Optional. Specify a reward id to only receive notifications for a specific reward.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub reward_id: Option<types::RewardId>,
}

impl ChannelPointsCustomRewardRemoveV1 {
    /// The broadcaster user ID for the channel you want to receive channel points custom reward remove notifications for.
    pub fn broadcaster_user_id(broadcaster_user_id: impl Into<types::UserId>) -> Self {
        Self {
            broadcaster_user_id: broadcaster_user_id.into(),
            reward_id: None,
        }
    }

    /// Specify a reward id to only receive notifications for a specific reward.
    pub fn reward_id(mut self, reward_id: impl Into<types::RewardId>) -> Self {
        self.reward_id = Some(reward_id.into());
        self
    }
}

impl EventSubscription for ChannelPointsCustomRewardRemoveV1 {
    type Payload = ChannelPointsCustomRewardRemoveV1Payload;

    const EVENT_TYPE: EventType = EventType::ChannelPointsCustomRewardRemove;
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![any(
        twitch_oauth2::Scope::ChannelReadRedemptions,
        twitch_oauth2::Scope::ChannelManageRedemptions
    )];
    const VERSION: &'static str = "1";
}

/// [`channel.channel_points_custom_reward.remove`](ChannelPointsCustomRewardRemoveV1) response payload.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct ChannelPointsCustomRewardRemoveV1Payload {
    /// Custom background color for the reward. Format: Hex with # prefix. Example: #FA1ED2.
    pub background_color: String,
    /// The requested broadcaster ID.
    pub broadcaster_user_id: types::UserId,
    /// The requested broadcaster login.
    pub broadcaster_user_login: types::UserName,
    /// The requested broadcaster display name.
    pub broadcaster_user_name: types::DisplayName,
    /// Timestamp of the cooldown expiration. null if the reward isn’t on cooldown.
    pub cooldown_expires_at: Option<types::Timestamp>,
    /// The reward cost.
    pub cost: i64,
    /// Set of default images of 1x, 2x and 4x sizes for the reward.
    pub default_image: Option<types::Image>,
    /// Whether a cooldown is enabled and what the cooldown is in seconds.
    pub global_cooldown: types::GlobalCooldown,
    /// The reward identifier.
    pub id: types::RewardId,
    /// Set of custom images of 1x, 2x and 4x sizes for the reward. Can be null if no images have been uploaded.
    pub image: Option<types::Image>,
    /// Is the reward currently enabled. If false, the reward won’t show up to viewers.
    pub is_enabled: bool,
    /// Is the reward currently in stock. If false, viewers can’t redeem.
    pub is_in_stock: bool,
    /// Is the reward currently paused. If true, viewers can’t redeem.
    pub is_paused: bool,
    /// Does the viewer need to enter information when redeeming the reward.
    pub is_user_input_required: bool,
    /// Whether a maximum per stream is enabled and what the maximum is.
    pub max_per_stream: types::Max,
    /// Whether a maximum per user per stream is enabled and what the maximum is.
    pub max_per_user_per_stream: types::Max,
    /// The reward description.
    pub prompt: String,
    /// The number of redemptions redeemed during the current live stream. Counts against the max_per_stream limit. null if the broadcasters stream isn’t live or max_per_stream isn’t enabled.
    pub redemptions_redeemed_current_stream: Option<u32>,
    /// Should redemptions be set to fulfilled status immediately when redeemed and skip the request queue instead of the normal unfulfilled status.
    pub should_redemptions_skip_request_queue: bool,
    /// The reward title.
    pub title: String,
}

#[cfg(test)]
#[test]
fn parse_payload() {
    // FIXME: Twitch reee. The condition `reward_id` is documented as a string, but in example is int
    let payload = r##"
    {
        "subscription": {
            "id": "f1c2a387-161a-49f9-a165-0f21d7a4e1c4",
            "type": "channel.channel_points_custom_reward.remove",
            "version": "1",
            "status": "enabled",
            "cost": 0,
            "condition": {
                "broadcaster_user_id": "1337",
                "reward_id": "12345"
            },
             "transport": {
                "method": "webhook",
                "callback": "https://example.com/webhooks/callback"
            },
            "created_at": "2019-11-16T10:11:12.123Z"
        },
        "event": {
            "id": "9001",
            "broadcaster_user_id": "1337",
            "broadcaster_user_login": "cool_user",
            "broadcaster_user_name": "Cool_User",
            "is_enabled": true,
            "is_paused": false,
            "is_in_stock": true,
            "title": "Cool Reward",
            "cost": 100,
            "prompt": "reward prompt",
            "is_user_input_required": true,
            "should_redemptions_skip_request_queue": false,
            "cooldown_expires_at": "2019-11-16T10:11:12.123Z",
            "redemptions_redeemed_current_stream": 123,
            "max_per_stream": {
                "is_enabled": true,
                "value": 1000
            },
            "max_per_user_per_stream": {
                "is_enabled": true,
                "value": 1000
            },
            "global_cooldown": {
                "is_enabled": true,
                "seconds": 1000
            },
            "background_color": "#FA1ED2",
            "image": {
                "url_1x": "https://static-cdn.jtvnw.net/image-1.png",
                "url_2x": "https://static-cdn.jtvnw.net/image-2.png",
                "url_4x": "https://static-cdn.jtvnw.net/image-4.png"
            },
            "default_image": {
                "url_1x": "https://static-cdn.jtvnw.net/default-1.png",
                "url_2x": "https://static-cdn.jtvnw.net/default-2.png",
                "url_4x": "https://static-cdn.jtvnw.net/default-4.png"
            }
        }
    }
    "##;

    let val = dbg!(crate::eventsub::Event::parse(payload).unwrap());
    crate::tests::roundtrip(&val)
}
