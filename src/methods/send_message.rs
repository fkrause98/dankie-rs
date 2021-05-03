use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, ParseMode, Text},
    },
};
use serde::Serialize;

/// Sends a text message.
///
/// Reflects the [`sendMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendMessage<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<message::Id>,
    allow_sending_without_reply: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendMessage<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        text: impl Into<Text>,
    ) -> Self {
        let text = text.into();

        Self {
            bot,
            chat_id: chat_id.into(),
            text: text.text,
            parse_mode: text.parse_mode,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    /// Configures if a preview for the first link in the message should be
    /// shown. Reflects the `disable_web_page_preview` parameter.
    pub const fn is_web_page_preview_disabled(
        mut self,
        is_disabled: bool,
    ) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }

    /// Configures if the message will be sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this text message is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub const fn in_reply_to(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
        self
    }

    /// Configures whether this message should be sent even
    /// if the replied-to message is not found.
    /// Reflects the `allow_sending_without_reply` parameter.
    pub const fn allow_sending_without_reply(mut self) -> Self {
        self.allow_sending_without_reply = true;
        self
    }

    /// Configures a keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(
        mut self,
        markup: impl Into<keyboard::Any<'a>>,
    ) -> Self {
        self.reply_markup = Some(markup.into());
        self
    }
}

impl SendMessage<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "sendMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
