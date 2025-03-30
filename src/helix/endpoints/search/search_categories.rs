//! Returns a list of games or categories that match the query via name either entirely or partially.
//! [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
//!
//! # Accessing the endpoint
//!
//! ## Request: [SearchCategoriesRequest]
//!
//! To use this endpoint, construct a [`SearchCategoriesRequest`] with the [`SearchCategoriesRequest::query()`] method.
//!
//! ```rust
//! use twitch_api::helix::search::search_categories;
//! let request = search_categories::SearchCategoriesRequest::query("hello");
//! ```
//!
//! ## Response: [Category](types::TwitchCategory)
//!
//! Send the request to receive the response with [`HelixClient::req_get()`](helix::HelixClient::req_get).
//!
//! ```rust, no_run
//! use twitch_api::helix::{self, search::search_categories};
//! # use twitch_api::client;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
//! # let client: helix::HelixClient<'static, client::DummyHttpClient> = helix::HelixClient::default();
//! # let token = twitch_oauth2::AccessToken::new("validtoken".to_string());
//! # let token = twitch_oauth2::UserToken::from_existing(&client, token, None, None).await?;
//! let request = search_categories::SearchCategoriesRequest::query("hello");
//! let response: Vec<search_categories::Category> = client.req_get(request, &token).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! You can also get the [`http::Request`] with [`request.create_request(&token, &client_id)`](helix::RequestGet::create_request)
//! and parse the [`http::Response`] with [`SearchCategoriesRequest::parse_response(None, &request.get_uri(), response)`](SearchCategoriesRequest::parse_response)
use super::*;
use helix::RequestGet;

// FIXME: One of id, user_id or game_id needs to be specified. typed_builder should have enums. id can not be used with other params
/// Query Parameters for [Search Categories](super::search_categories)
///
/// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
#[derive(PartialEq, Eq, Deserialize, Serialize, Clone, Debug)]
#[cfg_attr(feature = "typed-builder", derive(typed_builder::TypedBuilder))]
#[must_use]
#[non_exhaustive]
pub struct SearchCategoriesRequest<'a> {
    /// URI encoded search query
    #[cfg_attr(feature = "typed-builder", builder(setter(into)))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub query: Cow<'a, str>,
    /// Cursor for forward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub after: Option<Cow<'a, helix::CursorRef>>,
    /// Cursor for backward pagination: tells the server where to start fetching the next set of results, in a multi-page response. The cursor value specified here is from the pagination response field of a prior query.
    #[cfg_attr(feature = "typed-builder", builder(default))]
    #[cfg_attr(feature = "deser_borrow", serde(borrow = "'a"))]
    pub before: Option<Cow<'a, helix::CursorRef>>,
    /// Number of values to be returned per page. Limit: 100. Default: 20.
    #[cfg_attr(feature = "typed-builder", builder(setter(into), default))]
    pub first: Option<usize>,
}

impl<'a> SearchCategoriesRequest<'a> {
    /// Search categories with the following query.
    pub fn query(query: impl Into<Cow<'a, str>>) -> Self {
        Self {
            query: query.into(),
            after: None,
            before: None,
            first: None,
        }
    }

    /// Set amount of results returned per page.
    pub const fn first(mut self, first: usize) -> Self {
        self.first = Some(first);
        self
    }
}

/// Return Values for [Search Categories](super::search_categories)
///
/// [`search-categories`](https://dev.twitch.tv/docs/api/reference#search-categories)
pub type Category = types::TwitchCategory;

impl Request for SearchCategoriesRequest<'_> {
    type Response = Vec<Category>;

    const PATH: &'static str = "search/categories";
    #[cfg(feature = "twitch_oauth2")]
    const SCOPE: twitch_oauth2::Validator = twitch_oauth2::validator![];
}

impl RequestGet for SearchCategoriesRequest<'_> {
    fn parse_inner_response(
        request: Option<Self>,
        uri: &http::Uri,
        response: &str,
        status: http::StatusCode,
    ) -> Result<helix::Response<Self, Self::Response>, helix::HelixRequestGetError>
    where
        Self: Sized,
    {
        let response: helix::InnerResponse<Option<Self::Response>> =
            helix::parse_json(response, true).map_err(|e| {
                helix::HelixRequestGetError::DeserializeError(
                    response.to_string(),
                    e,
                    uri.clone(),
                    status,
                )
            })?;
        Ok(helix::Response::new(
            response.data.unwrap_or_default(),
            response.pagination.cursor,
            request,
            response.total,
            response.other,
        ))
    }
}

impl helix::Paginated for SearchCategoriesRequest<'_> {
    fn set_pagination(&mut self, cursor: Option<helix::Cursor>) {
        self.after = cursor.map(|c| c.into_cow())
    }
}

#[cfg(test)]
#[test]
fn test_request() {
    use helix::*;
    let req = SearchCategoriesRequest::query("fort");

    // From twitch docs
    let data = br#"
{
    "data": [
        {
            "id": "33214",
            "name": "Fortnite",
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-{width}x{height}.jpg"
        },
        {
            "id": "33214",
            "name": "Fortnite",
            "box_art_url": "https://static-cdn.jtvnw.net/ttv-boxart/Fortnite-{width}x{height}.jpg"
        }
    ],
    "pagination": {
        "cursor": "eyJiIjpudWxsLCJhIjp7IkN"
    }
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/search/categories?query=fort"
    );

    dbg!(SearchCategoriesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}

#[cfg(test)]
#[test]
fn test_request_null() {
    use helix::*;
    let req = SearchCategoriesRequest::query("aaaaaaaaaaaaaaaaaaaaaaaaaaa");

    // From twitch docs
    let data = br#"
{
    "data": null,
    "pagination": {}
}
"#
    .to_vec();

    let http_response = http::Response::builder().body(data).unwrap();

    let uri = req.get_uri().unwrap();
    assert_eq!(
        uri.to_string(),
        "https://api.twitch.tv/helix/search/categories?query=aaaaaaaaaaaaaaaaaaaaaaaaaaa"
    );

    dbg!(SearchCategoriesRequest::parse_response(Some(req), &uri, http_response).unwrap());
}
