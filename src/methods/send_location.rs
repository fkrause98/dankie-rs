use super::*;

/// Represents the [`sendLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendlocation
#[derive(Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendLocation<'a> {
    #[serde(skip)]
    token: Token,
    #[cfg(feature = "proxy")]
    #[serde(skip)]
    proxy: Option<proxy::Proxy>,
    chat_id: types::ChatId<'a>,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a> SendLocation<'a> {
    /// Constructs a new `SendLocation`.
    pub fn new(
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            token,
            chat_id: chat_id.into(),
            latitude,
            longitude,
            live_period: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
            #[cfg(feature = "proxy")]
            proxy: None,
        }
    }

    /// Confgiures `live_period`.
    pub fn live_period(mut self, duration: u32) -> Self {
        self.live_period = Some(duration);
        self
    }

    /// Confgiures `disable_notification`.
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Confgiures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: u32) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Confgiures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::AnyKeyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl IntoFuture for SendLocation<'_> {
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            &self.token,
            "sendLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
            #[cfg(feature = "proxy")]
            self.proxy,
        ))
    }
}

#[cfg(feature = "proxy")]
impl ProxyMethod for SendLocation<'_> {
    fn proxy(mut self, proxy: proxy::Proxy) -> Self {
        self.proxy = Some(proxy);
        self
    }
}
