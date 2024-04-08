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
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct PredictionsChannelV1Reply {
    /// Type of event
    #[serde(rename = "type")]
    pub type_field: String,
    /// Event data
    pub data: Data,
}

/// Event data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct Data {
    /// Event timestamp
    pub timestamp: types::Timestamp,
    /// Event
    pub event: Event,
}

/// Event
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Event {
    /// ID
    pub id: String,
    /// Channel ID
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// Created at
    #[serde(rename = "created_at")]
    pub created_at: types::Timestamp,
    /// Created by
    #[serde(rename = "created_by")]
    pub created_by: ByUser,
    /// Ended at
    #[serde(rename = "ended_at")]
    pub ended_at: Option<types::Timestamp>,
    /// Ended by user
    #[serde(rename = "ended_by")]
    pub ended_by: Option<ByUser>,
    /// Locked at
    #[serde(rename = "locked_at")]
    pub locked_at: Option<types::Timestamp>,
    /// Outcomes
    pub outcomes: Vec<Outcome>,
    /// Prediction window in seconds
    #[serde(rename = "prediction_window_seconds")]
    pub prediction_window_seconds: i64,
    /// Status
    pub status: String,
    /// Title
    pub title: String,
    /// Winning outcome ID
    #[serde(rename = "winning_outcome_id")]
    pub winning_outcome_id: Option<String>,
}

/// Created or ended by user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct ByUser {
    /// USER
    #[serde(rename = "type")]
    pub type_field: String,
    /// User ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// User display name
    #[serde(rename = "user_display_name")]
    pub user_display_name: String,
}

/// Outcome
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Outcome {
    /// ID
    pub id: String,
    /// Color
    pub color: String,
    /// Title
    pub title: String,
    /// Total prediction points
    #[serde(rename = "total_points")]
    pub total_points: i64,
    /// Total users
    #[serde(rename = "total_users")]
    pub total_users: i64,
    /// Top predictors
    #[serde(rename = "top_predictors")]
    pub top_predictors: Vec<TopPredictor>,
    /// Badge
    pub badge: Badge,
}

/// Top predictor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct TopPredictor {
    /// ID
    pub id: String,
    /// Event ID
    #[serde(rename = "event_id")]
    pub event_id: String,
    /// Outcome ID
    #[serde(rename = "outcome_id")]
    pub outcome_id: String,
    /// Channel ID
    #[serde(rename = "channel_id")]
    pub channel_id: String,
    /// Points
    pub points: i64,
    /// Predicted at
    #[serde(rename = "predicted_at")]
    pub predicted_at: types::Timestamp,
    /// Updated at
    #[serde(rename = "updated_at")]
    pub updated_at: types::Timestamp,
    /// User ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// User display name
    #[serde(rename = "user_display_name")]
    pub user_display_name: String,
}

/// Badge
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Badge {
    /// Version
    pub version: String,
    /// Set ID
    #[serde(rename = "set_id")]
    pub set_id: String,
}
