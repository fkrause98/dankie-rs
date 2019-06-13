use super::*;
use crate::internal::Client;
use parameters::NotificationState;
use types::input_file::{InputFile, Sticker};

/// Represents the [`sendSticker`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendsticker
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendSticker<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: types::ChatId<'a>,
    sticker: &'a Sticker<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<u32>,
    reply_markup: Option<types::AnyKeyboard<'a>>,
}

impl<'a, C> SendSticker<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl Into<types::ChatId<'a>>,
        sticker: &'a Sticker<'a>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            sticker,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures `disable_notification`.
    pub fn notification(mut self, state: NotificationState) -> Self {
        self.disable_notification = Some(state.is_disabled());
        self
    }

    /// Configures `reply_to_message_id`.
    pub fn reply_to_message_id(mut self, id: u32) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures `reply_markup`.
    pub fn reply_markup(
        mut self,
        markup: impl Into<types::AnyKeyboard<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl<C> IntoFuture for SendSticker<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future =
        Box<dyn Future<Item = Self::Item, Error = Self::Error> + Send>;
    type Item = types::Message;
    type Error = DeliveryError;

    fn into_future(self) -> Self::Future {
        let chat_id = match self.chat_id {
            types::ChatId::Id(id) => id.to_string(),
            types::ChatId::Username(username) => username.into(),
        };

        let is_disabled = self.disable_notification.map(|x| x.to_string());
        let reply_to = self.reply_to_message_id.map(|id| id.to_string());
        let reply_markup = self
            .reply_markup
            .and_then(|markup| serde_json::to_string(&markup).ok());

        let mut multipart = Multipart::new(5)
            .str("chat_id", &chat_id)
            .maybe_string("disabled_notification", &is_disabled)
            .maybe_string("reply_to_message_id", &reply_to)
            .maybe_string("reply_markup", &reply_markup);

        match self.sticker.media {
            InputFile::File {
                filename,
                bytes,
                ..
            } => multipart = multipart.file("sticker", filename, bytes),
            InputFile::Id(sticker) | InputFile::Url(sticker) => {
                multipart = multipart.str("sticker", sticker);
            }
        }

        let (boundary, body) = multipart.finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "sendSticker",
            Some(boundary),
            body,
        ))
    }
}
