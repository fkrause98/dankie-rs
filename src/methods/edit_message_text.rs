use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        keyboard::inline,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId, ParseMode, Text},
    },
};
use serde::Serialize;
use std::borrow::Cow;

/// Edits the text of a message sent by the bot itself.
///
/// Reflects the [`editMessageText`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagetext
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageText<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    message_id: message::Id,
    text: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditMessageText<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
        text: impl Into<Text<'a>>,
    ) -> Self {
        let text = text.into();

        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            text: text.text,
            parse_mode: text.parse_mode,
            disable_web_page_preview: None,
            reply_markup: None,
        }
    }

    /// Configures whether a preview for the first link in the message should be
    /// shown. Reflects the `disable_web_page_preview` parameter.
    pub const fn is_web_page_preview_disabled(
        mut self,
        is_disabled: bool,
    ) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditMessageText<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        call_method(
            self.bot,
            "editMessageText",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
