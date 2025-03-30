//! Gets information on follow relationships between two Twitch users.
//! [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
//!
//! ## Request: [GetUsersFollowsRequest]
//!
//! To use this endpoint, construct a [`GetUsersFollowsRequest`]
//!
//! ```rust
//! use twitch_api::helix::users::get_users_follows;
//! let request = get_users_follows::GetUsersFollowsRequest::following("1234");
//! ```
//!
//! ## Response: [UsersFollows]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, users::get_users_follows};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = get_users_follows::GetUsersFollowsRequest::following("1234");
//! let response: Vec<get_users_follows::FollowRelationship> = client.req_get(request, &token).await?.data.follow_relationships;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`GetUsersFollowsRequest::parse_response(None, &request.get_uri(), response)`](GetUsersFollowsRequest::parse_response)

use super::*;
use helix::RequestGet;
/// Query Parameters for [Get Users Follows](super::get_users_follows)
///
/// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[non_exhaustive]
#[deprecated(
    note = "this endpoint has been deprecated, see https://discuss.dev.twitch.tv/t/follows-endpoints-and-eventsub-subscription-type-are-now-available-in-open-beta/43322"
)]
pub struct GetUsersFollowsRequest<'a> {
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Maximum number of objects to return. Maximum: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    pub first: Option<usize>,
    /// User ID. The request returns information about users who are being followed by the from_id user.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub from_id: Option<Cow<'a, types::UserIdRef>>,
    /// User ID. The request returns information about users who are following the to_id user.
    #[cfg_attr(feature = "typed-builder", builder(default, setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub to_id: Option<Cow<'a, types::UserIdRef>>,
}

impl<'a> GetUsersFollowsRequest<'a> {
    /// Get the broadcasters that `from_id` is following
    pub fn following(from_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            from_id: Some(from_id.into_cow()),
            ..Self::empty()
        }
    }

    /// Get the followers of `to_id`
    pub fn followers(to_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            to_id: Some(to_id.into_cow()),
            ..Self::empty()
        }
    }

    /// Check if user follows a specific broadcaster
    pub fn follows(
        user_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
        broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a,
    ) -> Self {
        Self {
            from_id: Some(user_id.into_cow()),
            to_id: Some(broadcaster_id.into_cow()),
            ..Self::empty()
        }
    }

    /// Returns an empty [`GetUsersFollowsRequest`]
    ///
    /// # Notes
    ///
    /// This is not a valid request, it needs to be filled with other fields
    pub const fn empty() -> Self {
        Self {
            after: None,
            first: None,
            from_id: None,
            to_id: None,
        }
    }
}

/// Return Values for [Get Users Follows](super::get_users_follows)
///
/// [`get-users-follows`](https://dev.twitch.tv/docs/api/reference#get-users-follows)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
#[deprecated(
    note = "this endpoint has been deprecated, see https://discuss.dev.twitch.tv/t/follows-endpoints-and-eventsub-subscription-type-are-now-available-in-open-beta/43322"
)]
pub struct UsersFollows {
    /// Total number of items returned in all pages.
    ///
    /// * If only `from_id` was in the request, this is the total number of followed users.
    /// * If only `to_id` was in the request, this is the total number of followers.
    /// * If both `from_id` and `to_id` were in the request, this is 1 (if the "from" user follows the "to" user) or 0.
    pub total: i64,
    /// The follow relationships returned by this endpoint on this page. See [Response::get_next](helix::Response::get_next) for getting more pages
    pub follow_relationships: Vec<FollowRelationship>,
}
/// Describes a follow relationship
///
/// Used in [UsersFollows]
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct FollowRelationship {
    ///Date and time when the from_id user followed the to_id user.
    pub followed_at: types::Timestamp,
    ///ID of the user following the to_id user.
    pub from_id: types::UserId,
    ///Display name corresponding to from_id.
    pub from_name: types::DisplayName,
    /// Login of the user following the to_id user.
    pub from_login: types::UserName,
    ///ID of the user being followed by the from_id user.
    pub to_id: types::UserId,
    ///Display name corresponding to to_id.
    pub to_name: types::DisplayName,
    ///Login of the user being followed by the from_id user.
    pub to_login: types::UserName,
}

impl Request for GetUsersFollowsRequest<'_> {
    type Response = UsersFollows;

    #[cfg(feature = "twitch_oauth2")]
    const OPT_SCOPE: &'static [twitch_oauth2::Scope] = &[];
    const PATH: &'static str = "users/follows";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for GetUsersFollowsRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        #[derive(PartialEq, Deserialize, Debug, Clone)]
        struct InnerResponse {
            data: Vec<FollowRelationship>,
            total: i64,
            #[serde(default)]
            pagination: helix::Pagination,
        }

        let response: InnerResponse = helix::parse_json(response, true).map_err(|e| {
            helix::HelixRequestGetError::DeserializeError(
                response.to_string(),
                e,
                uri.clone(),
                status,
            )
        })?;
        Ok(helix::Response::new(
            UsersFollows {
                total: response.total,
                follow_relationships: response.data,
            },
            response.pagination.cursor,
            request,
            Some(response.total),
            None,
        ))
    }
}

impl helix::Paginated for GetUsersFollowsRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = GetUsersFollowsRequest::followers("23161357");
    // From twitch docs
    let data = br#"
{
    "total": 12345,
    "data":
    [
        {
            "from_id": "171003792",
            "from_login": "iiisutha067iii",
            "from_name": "IIIsutha067III",
            "to_id": "23161357",
            "to_name": "LIRIK",
            "to_login": "lirik",
            "followed_at": "2017-08-22T22:55:24Z"
        },
        {
            "from_id": "113627897",
            "from_login": "birdman616",
            "from_name": "Birdman616",
            "to_id": "23161357",
            "to_name": "LIRIK",
            "to_login": "lirik",
            "followed_at": "2017-08-22T22:55:04Z"
        }
    ],
    "pagination":{
        "cursor": "eyJiIjpudWxsLCJhIjoiMTUwMzQ0MTc3NjQyNDQyMjAwMCJ9"
    }
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/users/follows?to_id=23161357"
    );

    dbg!(GetUsersFollowsRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
