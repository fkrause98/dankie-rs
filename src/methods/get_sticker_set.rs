use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::sticker,
};

/// Represents the [`getStickerSet`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getstickerset
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetStickerSet<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    name: &'a str,
}

impl<'a, C> GetStickerSet<'a, C> {
    pub(crate) const fn new(
        client: &'a Client<C>,
        token: Token,
        name: &'a str,
    ) -> Self {
        Self {
            client,
            token,
            name,
        }
    }
}

impl<C> IntoFuture for GetStickerSet<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = sticker::Set;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getStickerSet",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
