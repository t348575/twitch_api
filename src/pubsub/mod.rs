//! Holds serializable pubsub stuff
//!
//! Use [`listen_command()`] to send subscription listen and parse the responses with [`Response::parse`]
//!
//! # Undocumented features
//!
//! This crate has some pubsub topics that are not documented by twitch. These may stop working at any time. To enable these, use feature
//! <span
//!   class="module-item stab portability"
//!   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
//! ><code>unsupported</code></span>
//! to use them. Note that this crate doesn't try to keep changes to these pubsub topics semver compatible.

static ERROR_TRYFROM: &str = "no match";

/// Implement `From<$type> for String` for serializing and `TryFrom<String> for $type` for deserializing.
macro_rules! impl_de_ser {
    (@field $e:expr) => {".{}"};
    ($type:ident, $fmt:literal, $($field:ident),* $(,)? $(?$opt_field:ident),* $(,)?) => {
        impl From<$type> for String {
            fn from(t: $type) -> Self { format!(concat!($fmt, $(impl_de_ser!(@field $field),)+ $(impl_de_ser!(@field $opt_field),)*), $(t.$field,)*$(t.$opt_field.map(|f| f.to_string()).unwrap_or_default(),)*).trim_end_matches(".").to_owned() }
        }
        impl<'a> From<&'a $type> for String {
            fn from(t: &'a $type) -> Self { format!(concat!($fmt, $(impl_de_ser!(@field $field),)+ $(impl_de_ser!(@field $opt_field),)*), $(t.$field,)*$(t.$opt_field.map(|f| f.to_string()).unwrap_or_default(),)*).trim_end_matches(".").to_owned() }
        }

        impl From<$type> for super::Topics {
            fn from(t: $type) -> Self {
                use super::Topic as _;
                t.into_topic()
            }
        }

        impl ::std::fmt::Display for $type {
            ///
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let s: String = ::std::convert::TryInto::try_into(self).map_err(|_| ::std::fmt::Error)?;
                f.write_str(&s)
            }
        }

        impl ::std::convert::TryFrom<String> for $type {
            type Error = &'static str;

            fn try_from(s: String) -> ::std::result::Result<Self, Self::Error> {
                if s.starts_with($fmt) {
                    let sub_s = s.strip_prefix($fmt).ok_or("could not strip str, this should never be hit")?;
                    match sub_s.split('.').collect::<Vec<_>>().as_slice() {
                        ["", $($field,)* $($opt_field,)*] => {
                            Ok($type {
                                $(
                                    $field: $field.parse()
                                            .map_err(|_| concat!("could not parse field <", stringify!($field), ">"))?,
                                )*
                                $(
                                    $opt_field: Some($opt_field.parse()
                                            .map_err(|_| concat!("could not parse field <", stringify!($opt_field), ">"))?),
                                )*
                            } )
                        }
                        #[allow(unreachable_patterns)]
                        ["", $($field,)*] => {
                            Ok($type {
                                $(
                                    $field: $field.parse()
                                            .map_err(|_| concat!("could not parse field <", stringify!($field), ">"))?,
                                )*
                                $(
                                    $opt_field: None,
                                )*
                            } )
                        }
                        _ => Err(crate::pubsub::ERROR_TRYFROM)
                    }
                } else {
                    Err(crate::pubsub::ERROR_TRYFROM)
                }
            }
        }
    };
}

use serde::Deserializer;
use serde_derive::{Deserialize, Serialize};

pub mod automod_queue;
pub mod channel_bits;
pub mod channel_bits_badge;
#[cfg(feature = "unsupported")]
pub mod channel_cheer;
pub mod channel_points;
#[cfg(feature = "unsupported")]
pub mod channel_sub_gifts;
pub mod channel_subscriptions;
#[cfg(feature = "unsupported")]
pub mod community_points;
#[cfg(feature = "unsupported")]
pub mod following;
#[cfg(feature = "unsupported")]
pub mod hypetrain;
pub mod moderation;
#[cfg(feature = "unsupported")]
pub mod raid;
pub mod user_moderation_notifications;
#[cfg(feature = "unsupported")]
pub mod video_playback;
#[cfg(feature = "unsupported")]
pub mod predictions;

use crate::parse_json;

/// A logical partition of messages that clients may subscribe to, to get messages.
///
/// also known as event
pub trait Topic: serde::Serialize + Into<String> {
    /// Scopes needed by this topic
    ///
    /// This constant
    /// <span
    ///   class="module-item stab portability"
    ///   style="display: inline; border-radius: 3px; padding: 2px; font-size: 80%; line-height: 1.2;"
    /// ><code>unsupported</code></span>
    #[cfg(feature = "twitch_oauth2")]
    #[cfg_attr(nightly, doc(cfg(feature = "twitch_oauth2")))]
    const SCOPE: twitch_oauth2::Validator;

    /// Convert this into a [`Topics`]
    fn into_topic(self) -> Topics;
}

/// All possible topics
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Clone, Hash)]
#[serde(untagged)]
#[non_exhaustive]
pub enum Topics {
    /// AutoMod flags a message as potentially inappropriate, and when a moderator takes action on a message.
    AutoModQueue(automod_queue::AutoModQueue),
    /// A user redeems an reward using channel points.
    #[cfg(feature = "unsupported")]
    CommunityPointsChannelV1(community_points::CommunityPointsChannelV1),
    /// Anyone cheers in a specified channel.
    ChannelBitsEventsV2(channel_bits::ChannelBitsEventsV2),
    /// Anyone shares a bit badge in a specified channel.
    ChannelBitsBadgeUnlocks(channel_bits_badge::ChannelBitsBadgeUnlocks),
    /// A user redeems a cheer with shared rewards.
    #[cfg(feature = "unsupported")]
    ChannelCheerEventsPublicV1(channel_cheer::ChannelCheerEventsPublicV1),
    /// A user gifts subs.
    #[cfg(feature = "unsupported")]
    ChannelSubGiftsV1(channel_sub_gifts::ChannelSubGiftsV1),
    /// A moderator performs an action in the channel.
    ChatModeratorActions(moderation::ChatModeratorActions),
    /// A user redeems an reward using channel points.
    ChannelPointsChannelV1(channel_points::ChannelPointsChannelV1),
    /// A subscription event happens in channel
    ChannelSubscribeEventsV1(channel_subscriptions::ChannelSubscribeEventsV1),
    /// Statistics about stream
    #[cfg(feature = "unsupported")]
    VideoPlayback(video_playback::VideoPlayback),
    /// Statistics about stream
    #[cfg(feature = "unsupported")]
    VideoPlaybackById(video_playback::VideoPlaybackById),
    /// A user redeems an reward using channel points.
    #[cfg(feature = "unsupported")]
    HypeTrainEventsV1(hypetrain::HypeTrainEventsV1),
    /// A user redeems an reward using channel points.
    #[cfg(feature = "unsupported")]
    HypeTrainEventsV1Rewards(hypetrain::HypeTrainEventsV1Rewards),
    /// A user follows the channel
    #[cfg(feature = "unsupported")]
    Following(following::Following),
    /// A user raids the channel
    #[cfg(feature = "unsupported")]
    Raid(raid::Raid),
    /// A user’s message held by AutoMod has been approved or denied.
    UserModerationNotifications(user_moderation_notifications::UserModerationNotifications),
    /// Channel predictions
    #[cfg(feature = "unsupported")]
    PredictionsChannelV1(predictions::PredictionsChannelV1),
    /// A user is awarded channel points
    #[cfg(feature = "unsupported")]
    CommunityPointsUserV1(community_points::CommunityPointsUserV1),
}

impl std::fmt::Display for Topics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Topics::*;
        let s = match self {
            AutoModQueue(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            CommunityPointsChannelV1(t) => t.to_string(),
            ChannelBitsEventsV2(t) => t.to_string(),
            ChannelBitsBadgeUnlocks(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            ChannelCheerEventsPublicV1(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            ChannelSubGiftsV1(t) => t.to_string(),
            ChatModeratorActions(t) => t.to_string(),
            ChannelPointsChannelV1(t) => t.to_string(),
            ChannelSubscribeEventsV1(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            VideoPlayback(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            VideoPlaybackById(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            HypeTrainEventsV1(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            HypeTrainEventsV1Rewards(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            Following(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            Raid(t) => t.to_string(),
            UserModerationNotifications(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            PredictionsChannelV1(t) => t.to_string(),
            #[cfg(feature = "unsupported")]
            CommunityPointsUserV1(t) => t.to_string(),
        };
        f.write_str(&s)
    }
}

#[derive(Serialize)]
struct ITopicSubscribeData<'a> {
    topics: &'a [String],
    #[serde(skip_serializing_if = "Option::is_none")]
    auth_token: Option<&'a str>,
}
#[derive(Serialize)]
struct ITopicSubscribe<'a> {
    #[serde(rename = "type")]
    _type: &'static str,
    nonce: Option<&'a str>,
    data: ITopicSubscribeData<'a>,
}

/// Create a listen command.
///
/// # Examples
///
/// Create a listen message for moderator actions
///
/// ```rust
/// # use twitch_api::pubsub::{self, Topic as _};
/// // We want to subscribe to moderator actions on channel with id 1234
/// // as if we were a user with id 4321 that is moderator on the channel.
/// let chat_mod_actions = pubsub::moderation::ChatModeratorActions {
///     user_id: 4321,
///     channel_id: 1234,
/// }
/// .into_topic();
///
/// // Listen to follows as well
/// let follows =
///     pubsub::following::Following { channel_id: 1234 }.into_topic();
/// // Create the topic command to send to twitch
/// let command = pubsub::listen_command(
///     &[chat_mod_actions, follows],
///     "authtoken",
///     "super se3re7 random string",
/// )
/// .expect("serializing failed");
/// // Send the message with your favorite websocket client
/// send_command(command).unwrap();
/// // To parse the websocket messages, use pubsub::Response::parse
/// # fn send_command(command: String) -> Result<(),()> {Ok(())}
/// ```
pub fn listen_command<'t, T, N>(
    topics: &'t [Topics],
    auth_token: T,
    nonce: N,
) -> Result<String, serde_json::Error>
where
    T: Into<Option<&'t str>>,
    N: Into<Option<&'t str>>,
{
    let topics = topics.iter().map(|t| t.to_string()).collect::<Vec<_>>();
    serde_json::to_string(&ITopicSubscribe {
        _type: "LISTEN",
        nonce: nonce.into(),
        data: ITopicSubscribeData {
            topics: &topics,
            auth_token: auth_token.into(),
        },
    })
}

/// Create a unlisten command.
///
/// # Examples
///
/// Unlisten from moderator actions and follows
///
/// ```rust
/// # use twitch_api::pubsub::{self, Topic as _};
/// // These are the exact same topics as for the `listen_command`.
/// let chat_mod_actions = pubsub::moderation::ChatModeratorActions {
///     user_id: 4321,
///     channel_id: 1234,
/// }
/// .into_topic();
///
/// let follows =
///     pubsub::following::Following { channel_id: 1234 }.into_topic();
/// // Create the command to send to twitch
/// let command = pubsub::unlisten_command(
///     &[chat_mod_actions, follows],
///     // This does not need to be the same nonce that was sent for listening.
///     // The nonce is only there to identify the payload and the response.
///     "super se3re7 random string",
/// )
/// .expect("serializing failed");
/// // Send the message with your favorite websocket client
/// send_command(command).unwrap();
/// // To parse the websocket messages, use pubsub::Response::parse
/// # fn send_command(command: String) -> Result<(),()> {Ok(())}
/// ```
pub fn unlisten_command<'t, O>(
    topics: &'t [Topics],
    nonce: O,
) -> Result<String, serde_json::Error>
where
    O: Into<Option<&'t str>>,
{
    let topics = topics.iter().map(|t| t.to_string()).collect::<Vec<_>>();
    serde_json::to_string(&ITopicSubscribe {
        _type: "UNLISTEN",
        nonce: nonce.into(),
        data: ITopicSubscribeData {
            topics: &topics,
            auth_token: None,
        },
    })
}

/// Response from twitch PubSub
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct TwitchResponse {
    /// The nonce that was passed in the request, if one was provided there
    pub nonce: Option<String>,
    /// The error message associated with the request, or an empty string if there is no error
    pub error: Option<String>,
}

impl TwitchResponse {
    /// Whether response indicates success or not
    pub fn is_successful(&self) -> bool { self.error.as_ref().map_or(true, |s| s.is_empty()) }
}

// FIXME: Add example
/// Message response from twitch PubSub.
///
/// See [TwitchResponse]
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum TopicData {
    /// Response from the [automod_queue::AutoModQueue] topic.
    AutoModQueue {
        /// Topic message
        topic: automod_queue::AutoModQueue,
        /// Message reply from topic subscription
        reply: Box<automod_queue::AutoModQueueReply>,
    },
    /// Response from the [channel_bits::ChannelBitsEventsV2] topic.
    ChannelBitsEventsV2 {
        /// Topic message
        topic: channel_bits::ChannelBitsEventsV2,
        /// Message reply from topic subscription
        reply: Box<channel_bits::ChannelBitsEventsV2Reply>,
    },
    /// Response from the [channel_bits_badge::ChannelBitsBadgeUnlocks] topic.
    ChannelBitsBadgeUnlocks {
        /// Topic message
        topic: channel_bits_badge::ChannelBitsBadgeUnlocks,
        /// Message reply from topic subscription
        reply: Box<channel_bits_badge::ChannelBitsBadgeUnlocksReply>,
    },
    /// Response from the [moderation::ChatModeratorActions] topic.
    ChatModeratorActions {
        /// Topic message
        topic: moderation::ChatModeratorActions,
        /// Message reply from topic subscription
        reply: Box<moderation::ChatModeratorActionsReply>,
    },
    /// Response from the [channel_points::ChannelPointsChannelV1] topic.
    ChannelPointsChannelV1 {
        /// Topic message
        topic: channel_points::ChannelPointsChannelV1,
        /// Message reply from topic subscription
        reply: Box<channel_points::ChannelPointsChannelV1Reply>,
    },
    /// Response from the [channel_subscriptions::ChannelSubscribeEventsV1] topic.
    ChannelSubscribeEventsV1 {
        /// Topic message
        topic: channel_subscriptions::ChannelSubscribeEventsV1,
        /// Message reply from topic subscription
        reply: Box<channel_subscriptions::ChannelSubscribeEventsV1Reply>, // FIXME: :)
    },
    /// Response from the [community_points::CommunityPointsChannelV1] topic.
    #[cfg(feature = "unsupported")]
    CommunityPointsChannelV1 {
        /// Topic message
        topic: community_points::CommunityPointsChannelV1,
        /// Message reply from topic subscription
        reply: Box<channel_points::ChannelPointsChannelV1Reply>,
    },
    /// Response from the [channel_cheer::ChannelCheerEventsPublicV1] topic.
    #[cfg(feature = "unsupported")]
    ChannelCheerEventsPublicV1 {
        /// Topic message
        topic: channel_cheer::ChannelCheerEventsPublicV1,
        /// Message reply from topic subscription
        reply: Box<channel_cheer::ChannelCheerEventsPublicV1Reply>,
    },
    /// Response from the [channel_sub_gifts::ChannelSubGiftsV1] topic.
    #[cfg(feature = "unsupported")]
    ChannelSubGiftsV1 {
        /// Topic message
        topic: channel_sub_gifts::ChannelSubGiftsV1,
        /// Message reply from topic subscription
        reply: Box<channel_sub_gifts::ChannelSubGiftsV1Reply>,
    },

    /// Response from the [video_playback::VideoPlayback] topic.
    #[cfg(feature = "unsupported")]
    VideoPlayback {
        /// Topic message
        topic: video_playback::VideoPlayback,
        /// Message reply from topic subscription
        reply: Box<video_playback::VideoPlaybackReply>,
    },
    /// Response from the [video_playback::VideoPlaybackById] topic.
    #[cfg(feature = "unsupported")]
    VideoPlaybackById {
        /// Topic message
        topic: video_playback::VideoPlaybackById,
        /// Message reply from topic subscription
        reply: Box<video_playback::VideoPlaybackReply>,
    },
    /// Response from the [hypetrain::HypeTrainEventsV1] topic.
    #[cfg(feature = "unsupported")]
    HypeTrainEventsV1 {
        /// Topic message
        topic: hypetrain::HypeTrainEventsV1,
        /// Message reply from topic subscription
        reply: Box<hypetrain::HypeTrainEventsV1Reply>, // FIXME: May not be correct
    },
    /// Response from the [hypetrain::HypeTrainEventsV1Rewards] topic.
    #[cfg(feature = "unsupported")]
    HypeTrainEventsV1Rewards {
        /// Topic message
        topic: hypetrain::HypeTrainEventsV1Rewards,
        /// Message reply from topic subscription
        reply: Box<hypetrain::HypeTrainEventsV1Reply>,
    },
    /// Response from the [following::Following] topic.
    #[cfg(feature = "unsupported")]
    Following {
        /// Topic message
        topic: following::Following,
        /// Message reply from topic subscription
        reply: Box<following::FollowingReply>,
    },
    /// Response from the [raid::Raid] topic.
    #[cfg(feature = "unsupported")]
    Raid {
        /// Topic message
        topic: raid::Raid,
        /// Message reply from topic subscription
        reply: Box<raid::RaidReply>,
    },
    /// A user’s message held by AutoMod has been approved or denied.
    UserModerationNotifications {
        /// Topic message
        topic: user_moderation_notifications::UserModerationNotifications,
        /// Message reply from topic subscription
        reply: Box<user_moderation_notifications::UserModerationNotificationsReply>,
    },
    /// Channel predictions
    #[cfg(feature = "unsupported")]
    PredictionsChannelV1 {
        /// Topic message
        topic: predictions::PredictionsChannelV1,
        /// Message reply from topic subscription
        reply: Box<predictions::PredictionsChannelV1Reply>,
    },
    /// A user is awarded channel points
    #[cfg(feature = "unsupported")]
    CommunityPointsUserV1 {
        /// Topic message
        topic: community_points::CommunityPointsUserV1,
        /// Message reply from topic subscription
        reply: Box<community_points::CommunityPointsUserV1Reply>,
    }
}

impl serde::Serialize for TopicData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        #[derive(Serialize, Debug)]
        struct ITopicData {
            topic: Topics,
            message: String,
        }

        let (topic, reply) = match self {
            TopicData::AutoModQueue { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::ChannelBitsEventsV2 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::ChannelBitsBadgeUnlocks { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::ChatModeratorActions { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::ChannelPointsChannelV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::ChannelSubscribeEventsV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::CommunityPointsChannelV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::ChannelCheerEventsPublicV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::ChannelSubGiftsV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::VideoPlayback { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::VideoPlaybackById { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::HypeTrainEventsV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::HypeTrainEventsV1Rewards { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::Following { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::Raid { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::UserModerationNotifications { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::PredictionsChannelV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
            TopicData::CommunityPointsUserV1 { topic, reply } => (topic.clone().into_topic(), serde_json::to_string(&reply).map_err(serde::ser::Error::custom)?),
        };

        ITopicData { topic, message: reply }.serialize(serializer)
    }
}

// This impl is here because otherwise we hide the errors from deser
impl<'de> serde::Deserialize<'de> for TopicData {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // FIXME: make into macro or actually upstream into serde..., untagged_force = "field"

        #[derive(Deserialize, Debug)]
        struct ITopicData {
            topic: Topics,
            message: String,
        }
        let reply = ITopicData::deserialize(deserializer).map_err(|e| {
            serde::de::Error::custom(format!("could not deserialize topic reply: {e}"))
        })?;
        Ok(match reply.topic {
            Topics::AutoModQueue(topic) => TopicData::AutoModQueue {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::CommunityPointsChannelV1(topic) => TopicData::CommunityPointsChannelV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            Topics::ChannelBitsEventsV2(topic) => TopicData::ChannelBitsEventsV2 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            Topics::ChannelBitsBadgeUnlocks(topic) => TopicData::ChannelBitsBadgeUnlocks {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::ChannelSubGiftsV1(topic) => TopicData::ChannelSubGiftsV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::ChannelCheerEventsPublicV1(topic) => TopicData::ChannelCheerEventsPublicV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            Topics::ChatModeratorActions(topic) => TopicData::ChatModeratorActions {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            Topics::ChannelPointsChannelV1(topic) => TopicData::ChannelPointsChannelV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            Topics::ChannelSubscribeEventsV1(topic) => TopicData::ChannelSubscribeEventsV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::VideoPlayback(topic) => TopicData::VideoPlayback {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::VideoPlaybackById(topic) => TopicData::VideoPlaybackById {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::HypeTrainEventsV1(topic) => TopicData::HypeTrainEventsV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::HypeTrainEventsV1Rewards(topic) => TopicData::HypeTrainEventsV1Rewards {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::Following(topic) => TopicData::Following {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::Raid(topic) => TopicData::Raid {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            Topics::UserModerationNotifications(topic) => TopicData::UserModerationNotifications {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::PredictionsChannelV1(topic) => TopicData::PredictionsChannelV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            },
            #[cfg(feature = "unsupported")]
            Topics::CommunityPointsUserV1(topic) => TopicData::CommunityPointsUserV1 {
                topic,
                reply: parse_json(&reply.message, true).map_err(serde::de::Error::custom)?,
            }
        })
    }
}

/// Requests to the server, meant for mocking
/// #[derive(Clone, Debug, PartialEq, Deserialize)]
#[cfg(feature = "mock")]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Request {
    #[serde(rename = "PING")]
    /// Ping
    Ping,
    #[serde(rename = "LISTEN")]
    /// Listening to a topic
    Listen {
        /// Topics to listen to
        data: ListenData,
        /// Message nonce
        nonce: Option<String>
    },
    #[serde(rename = "UNLISTEN")]
    /// Listening to a topic
    UnListen {
        /// Topics to listen to
        data: ListenData,
        /// Message nonce
        nonce: Option<String>
    },
}

#[cfg(feature = "mock")]
/// Request to listen to a topic
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ListenData {
    /// Topics to listen to
    pub topics: Vec<Topics>,
    /// Authentication token
    pub auth_token: Option<String>
}

impl Request {
    // FIXME: Add example
    /// Parse string slice as a response.
    pub fn parse(source: &str) -> Result<Request, crate::DeserError> { parse_json(source, true) }
}

/// Response from twitchs PubSub server.
/// Either a response indicating status of something or a message from a topic
#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum Response {
    /// Response from a subscription/unsubscription
    #[serde(rename = "RESPONSE")]
    Response(TwitchResponse),
    /// Message received containing all applicable data
    #[serde(rename = "MESSAGE")]
    Message {
        /// Data corresponding to [topic](Topic) message
        data: TopicData,
    },
    /// Response from a ping
    #[serde(rename = "PONG")]
    Pong,
    /// Request for the client to reconnect
    #[serde(rename = "RECONNECT")]
    Reconnect,
}

impl Response {
    // FIXME: Add example
    /// Parse string slice as a response.
    pub fn parse(source: &str) -> Result<Response, crate::DeserError> { parse_json(source, true) }
}

/// Deserialize 'null' as <T as Default>::Default
fn deserialize_default_from_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: serde::Deserialize<'de> + Default, {
    use serde::Deserialize;

    Ok(Option::deserialize(deserializer)?.unwrap_or_default())
}

/// Deserialize "" as <T as Default>::Default
fn deserialize_none_from_empty_string<'de, D, S>(deserializer: D) -> Result<Option<S>, D::Error>
where
    D: serde::Deserializer<'de>,
    S: serde::Deserialize<'de>, {
    use serde::de::IntoDeserializer;
    struct Inner<S>(std::marker::PhantomData<S>);
    impl<'de, S> serde::de::Visitor<'de> for Inner<S>
    where S: serde::Deserialize<'de>
    {
        type Value = Option<S>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            formatter.write_str("any string")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: serde::de::Error {
            match value {
                "" => Ok(None),
                v => S::deserialize(v.into_deserializer()).map(Some),
            }
        }

        fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
        where E: serde::de::Error {
            match &*value {
                "" => Ok(None),
                v => S::deserialize(v.into_deserializer()).map(Some),
            }
        }

        fn visit_unit<E>(self) -> Result<Self::Value, E>
        where E: serde::de::Error {
            Ok(None)
        }
    }

    deserializer.deserialize_any(Inner(std::marker::PhantomData))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn error() {
        let source = r#"
{
    "type": "RESPONSE",
    "nonce": "44h1k13746815ab1r2",
    "error": ""
}
"#;
        let expected = Response::Response(TwitchResponse {
            nonce: Some(String::from("44h1k13746815ab1r2")),
            error: Some(String::new()),
        });
        let actual = Response::parse(source).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn listen() {
        let topic =
            Topics::ChannelBitsEventsV2(channel_bits::ChannelBitsEventsV2 { channel_id: 12345 });
        let expected = r#"{"type":"LISTEN","nonce":"my nonce","data":{"topics":["channel-bits-events-v2.12345"],"auth_token":"my token"}}"#;
        let actual = listen_command(&[topic], "my token", "my nonce").expect("should serialize");
        assert_eq!(expected, actual);
    }

    #[test]
    fn unlisten() {
        let topic =
            Topics::ChannelBitsEventsV2(channel_bits::ChannelBitsEventsV2 { channel_id: 12345 });
        let expected = r#"{"type":"UNLISTEN","nonce":"my nonce","data":{"topics":["channel-bits-events-v2.12345"]}}"#;
        let actual = unlisten_command(&[topic], "my nonce").expect("should serialize");
        assert_eq!(expected, actual);
    }
}
