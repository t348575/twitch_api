#![doc(alias = "prediction")]
#![doc(alias = "predictions-channel-v1")]
//! PubSub messages for predictions
use crate::{pubsub, types};
use serde::{Deserialize, Serialize};

/// A user redeems an reward using channel points.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct PredictionsChannelV1 {
    /// The channel_id to watch
    pub channel_id: u32,
}

impl_de_ser!(
    PredictionsChannelV1,
    "predictions-channel-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for PredictionsChannelV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];

    fn into_topic(self) -> pubsub::Topics { super::Topics::PredictionsChannelV1(self) }
}

/// Reply from [PredictionsChannelV1]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "type", content = "data")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub enum PredictionsChannelV1Reply {}