//! Helix endpoints regarding subscriptions
//!
//! # Implemented endpoints
//!
//! <!-- generate with "cargo xtask overview" (with a nightly toolchain) -->
//! <!-- BEGIN-OVERVIEW -->
//! <details open><summary style="cursor: pointer">Teams 🟢 2/2</summary>
//!
//! | Endpoint | Helper | Module |
//! |---|---|---|
//! | [Get Channel Teams](https://dev.twitch.tv/docs/api/reference#get-channel-teams) | - | [`get_channel_teams`] |
//! | [Get Teams](https://dev.twitch.tv/docs/api/reference#get-teams) | - | [`get_teams`] |
//!
//! </details>
//!
//! <!-- END-OVERVIEW -->

use crate::{
    helix::{self, Request},
    types,
};
use serde_derive::{Deserialize, Serialize};
use std::borrow::Cow;

pub mod get_channel_teams;
pub mod get_teams;

/// Information about a team. Get team `members` with [`GetChannelTeamsRequest`](get_channel_teams::GetChannelTeamsRequest)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct TeamInformation {
    /// URL of the Team background `image`.
    pub background_image_url: Option<String>,
    /// URL for the Team banner.
    pub banner: Option<String>,
    /// Date and time the Team was created.
    pub created_at: types::Timestamp,
    /// Date and time the Team was last updated.
    pub updated_at: types::Timestamp,
    /// Team description.
    pub info: String,
    /// Image URL for the Team logo.
    pub thumbnail_url: Option<String>,
    /// Team name.
    pub team_name: String,
    /// Team display name.
    pub team_display_name: String,
    /// Team ID.
    pub id: types::TeamId,
}
