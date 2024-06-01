#![doc(alias = "prediction")]
#![doc(alias = "predictions-channel-v1")]
//! PubSub messages for predictions
use crate::{pubsub, types::{Timestamp, UserId}};
use serde_derive::{Deserialize, Serialize};

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

/// A user redeems an reward using channel points.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(into = "String", try_from = "String")]
pub struct PredictionsUserV1 {
    /// The channel_id to watch
    pub channel_id: u32,
}

impl_de_ser!(
    PredictionsUserV1,
    "predictions-user-v1",
    channel_id // FIXME: add trailing comma
);

impl pubsub::Topic for PredictionsUserV1 {
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];

    fn into_topic(self) -> pubsub::Topics { super::Topics::PredictionsUserV1(self) }
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
    pub timestamp: Timestamp,
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
    pub created_at: Timestamp,
    /// Ended at
    #[serde(rename = "ended_at")]
    pub ended_at: Option<Timestamp>,
    /// Locked at
    #[serde(rename = "locked_at")]
    pub locked_at: Option<Timestamp>,
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
    pub predicted_at: Timestamp,
    /// Updated at
    #[serde(rename = "updated_at")]
    pub updated_at: Timestamp,
    /// User ID
    #[serde(rename = "user_id")]
    pub user_id: String,
    /// User display name
    #[serde(rename = "user_display_name")]
    pub user_display_name: String,
}

/// Prediction made & result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct PredictionsUserV1Reply {
    /// Type of event, prediction-made or prediction-result
    #[serde(rename = "type")]
    pub type_field: String,
    /// Event data
    pub data: UserV1Data,
}

/// UserV1Data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
pub struct UserV1Data {
    /// Event timestamp
    pub timestamp: Timestamp,
    /// Prediction data
    pub prediction: Prediction,
}

/// Prediction data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Prediction {
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
    pub predicted_at: Timestamp,
    /// Updated at
    #[serde(rename = "updated_at")]
    pub updated_at: Timestamp,
    /// User ID
    #[serde(rename = "user_id")]
    pub user_id: UserId,
    /// Result
    pub result: Option<Result>,
    /// User display name
    #[serde(rename = "user_display_name")]
    pub user_display_name: Option<String>,
}

/// Result
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[cfg_attr(feature = "utoipa", derive(utoipa::ToSchema))]
pub struct Result {
    /// WIN or LOSS
    #[serde(rename = "type")]
    pub type_field: String,
    /// Points won
    #[serde(rename = "points_won")]
    pub points_won: Option<i64>,
    /// Is acknowledged
    #[serde(rename = "is_acknowledged")]
    pub is_acknowledged: bool,
}