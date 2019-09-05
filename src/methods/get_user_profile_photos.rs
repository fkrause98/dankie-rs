use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::user,
};

/// Gets a user's profile photos.
///
/// Reflects the [`getUserProfilePhotos`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getuserprofilephotos
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetUserProfilePhotos<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    user_id: user::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
}

impl<'a, C> GetUserProfilePhotos<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Configures the number of the first photo to be returned.
    /// Reflects the `offset` parameter.
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Configures how many photos will be returned. Must be in the range
    /// `1..=100`; defaults to 100. Reflects the `limit` parameter.
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl<C> IntoFuture for GetUserProfilePhotos<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = user::ProfilePhotos;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getUserProfilePhotos",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
