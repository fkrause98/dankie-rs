use super::*;

/// Representation of the [`sendMessage`] method.
///
/// [`sendMessage`]: https://core.telegram.org/bots/api#sendmessage
#[derive(Serialize)]
pub struct SendMessage<'a> {
    #[serde(skip)]
    token: &'a str,
    chat_id: types::ChatId<'a>,
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<types::ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<u64>,
    // TODO: Implement `reply_markup`
}

impl<'a> SendMessage<'a> {
    /// Creates a new `SendMessage`.
    #[must_use]
    pub fn new<'b: 'a>(
        token: &'b str,
        chat_id: impl Into<types::ChatId<'b>>,
        text: &'a str,
    ) -> SendMessage<'a> {
        SendMessage {
            token,
            chat_id: chat_id.into(),
            text,
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
        }
    }

    /// Sets `parse_mode` to `Some(mode)`.
    #[must_use]
    pub fn parse_mode(mut self, mode: types::ParseMode) -> Self {
        self.parse_mode = Some(mode);
        self
    }

    /// Sets `disable_web_page_preview` to `Some(is_disabled)`.
    #[must_use]
    pub fn disable_web_page_preview(mut self, is_disabled: bool) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }

    /// Sets `disable_notification` to `Some(is_disabled)`.
    #[must_use]
    pub fn disable_notification(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Sets `reply_to_message_id` to `Some(id)`.
    #[must_use]
    pub fn reply_to_message_id(mut self, id: u64) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Prepares the request and returns a `Future`.
    #[must_use]
    pub fn into_future(
        self,
    ) -> impl Future<Item = types::raw::Message, Error = DeliveryError> {
        send_method::<types::raw::Message>(
            self.token,
            "sendMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
    }
}
