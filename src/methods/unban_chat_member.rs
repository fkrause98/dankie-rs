use super::*;

/// Represents the [`unbanChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#unbanchatmember
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct UnbanChatMember<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    user_id: i64,
}

impl<'a> UnbanChatMember<'a> {
    /// Constructs a new `UnbanChatMember`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        user_id: i64,
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            user_id,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }
}

impl IntoFuture for UnbanChatMember<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = ();
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(
            send_method::<bool>(
                &self.token,
                "unbanChatMember",
                None,
                serde_json::to_vec(&self).unwrap(),
                #[cfg(feature = "proxy")]
                self.proxy,
            )
            .map(|_| ()), // Only `true` is returned on success
        )
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for UnbanChatMember<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
