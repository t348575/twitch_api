//! Checks if a specific user is subscribed to a specific channel.
//! [`check-user-subscription`](https://dev.twitch.tv/docs/api/reference#check-user-subscription)
//!
//! # Accessing the endpoint
//!
//! ## Request: [CheckUserSubscriptionRequest]
//!
//! To use this endpoint, construct a [`CheckUserSubscriptionRequest`] with the [`CheckUserSubscriptionRequest::broadcaster_id()`] method.
//!
//! ```rust
//! use twitch_api::helix::subscriptions::check_user_subscription;
//! let request =
//!     check_user_subscription::CheckUserSubscriptionRequest::broadcaster_id(
//!         "1234",
//!     );
//! ```
//!
//! ## Response: [UserSubscription]
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, subscriptions::check_user_subscription};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = check_user_subscription::CheckUserSubscriptionRequest::broadcaster_id("1234");
//! let response: check_user_subscription::UserSubscription = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`CheckUserSubscriptionRequest::parse_response(None, &request.get_uri(), response)`](CheckUserSubscriptionRequest::parse_response)
use super::*;
use helix::RequestGet;

/// Query Parameters for [Check User Subscription](super::check_user_subscription)
///
/// [`check-user-subscription`](https://dev.twitch.tv/docs/api/reference#check-user-subscription)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct CheckUserSubscriptionRequest<'a> {
    /// User ID of the broadcaster. Must match the User ID in the Bearer token.
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub broadcaster_id: Cow<'a, types::UserIdRef>,
    /// Unique identifier of account to get subscription status of. Accepts up to 100 values.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    // FIXME: This is essentially the same as borrow, but worse
    #[cfg_attr(not(feature = "deser_borrow"), serde(bound(deserialize = "'de: 'a")))]
    pub user_id: types::Collection<'a, types::UserId>,
}

impl<'a> CheckUserSubscriptionRequest<'a> {
    /// Checks subscribed users to this specific channel.
    pub fn broadcaster_id(broadcaster_id: impl types::IntoCow<'a, types::UserIdRef> + 'a) -> Self {
        Self {
            broadcaster_id: broadcaster_id.into_cow(),
            user_id: types::Collection::default(),
        }
    }

    /// Filter the results for specific users.
    pub fn user_ids(mut self, user_ids: impl Into<types::Collection<'a, types::UserId>>) -> Self {
        self.user_id = user_ids.into();
        self
    }
}

/// Return Values for [Check User Subscription](super::check_user_subscription)
///
/// [`check-user-subscription`](https://dev.twitch.tv/docs/api/reference#check-user-subscription)
#[derive(PartialEq, Eq, Deserialize, Serialize, Debug, Clone)]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
#[non_exhaustive]
pub struct UserSubscription {
    /// User ID of the broadcaster.
    pub broadcaster_id: types::UserId,
    /// Login of the broadcaster.
    pub broadcaster_login: types::UserName,
    /// Display name of the broadcaster.
    pub broadcaster_name: types::DisplayName,
    /// Indicates if the subscription is a gift.
    pub is_gift: bool,
    /// Login of the gifter (if is_gift is true).
    pub gifter_login: Option<types::UserName>,
    /// Display name of the gifter (if is_gift is true).
    pub gifter_name: Option<types::DisplayName>,
    /// Subscription tier. 1000 is tier 1, 2000 is tier 2, and 3000 is tier 3.
    pub tier: types::SubscriptionTier,
}

impl Request for CheckUserSubscriptionRequest<'_> {
    type Response = UserSubscription;

    const PATH: &'static str = "subscriptions/user";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator =
        twitch_oauth2::validator![twitch_oauth2::Scope::UserReadSubscriptions];
}

impl RequestGet for CheckUserSubscriptionRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        text: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let inner_response: helix::InnerResponse<Vec<_>> =
            helix::parse_json(text, true).map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(
                    text.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response::new(
            inner_response.data.into_iter().next().ok_or(
                helix::HelixRequestGetError::InvalidResponse {
                    reason: "expected an entry in `data`",
                    response: text.to_string(),
                    status,
                    uri: uri.clone(),
                },
            )?,
            inner_response.pagination.cursor,
            request,
            inner_response.total,
            inner_response.other,
        ))
    }
}

#[cfg(test)]
#[test]
fn test_request1() {
    use helix::*;
    let req = CheckUserSubscriptionRequest::broadcaster_id("123");

    // From twitch docs.
    let data = br#"
    {
        "data": [
          {
            "broadcaster_id": "149747285",
            "broadcaster_name": "TwitchPresents",
            "broadcaster_login": "twitchpresents",
            "is_gift": false,
            "tier": "1000"
          }
        ]
      }
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/subscriptions/user?broadcaster_id=123"
    );

    dbg!(CheckUserSubscriptionRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request2() {
    use helix::*;
    let req = CheckUserSubscriptionRequest::broadcaster_id("123");

    // From twitch docs.
    let data = br#"
    {
        "error": "Not Found",
        "message": "twitchdev has no subscription to twitchpresents",
        "status": 404
      }
"#
    .to_vec();

    let http_response = http::Response::builder().status(404).body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/subscriptions/user?broadcaster_id=123"
    );

    dbg!(CheckUserSubscriptionRequest::parse_response(Some(req), &uri, http_response).unwrap_err());
}
