use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        file,
        input_file::{Document, InputFile, Thumb},
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
    Multipart,
};

/// Sends a document.
///
/// Reflects the [`sendDocument`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#senddocument
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendDocument<'a> {
    bot: &'a InnerBot,
    chat_id: ChatId<'a>,
    document: Document<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendDocument<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId<'a>,
        document: Document<'a>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            document,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this document is sent in reply to.
    /// Reflects the `reply_to_message_id` parameter.
    pub const fn reply_to_message_id(mut self, id: message::Id) -> Self {
        self.reply_to_message_id = Some(id);
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

impl SendDocument<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        let mut multipart = Multipart::new(8)
            .chat_id("chat_id", self.chat_id)
            .maybe_str("caption", self.document.caption)
            .maybe_string("parse_mode", self.document.parse_mode)
            .maybe_string("disable_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match self.document.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("document", filename, bytes),
            InputFile::Id(file::id::Ref(document))
            | InputFile::Url(document) => {
                multipart = multipart.str("document", document);
            }
        }

        if let Some(Thumb(InputFile::File {
            filename, bytes, ..
        })) = self.document.thumb
        {
            multipart = multipart.file("thumb", filename, bytes);
        }

        let (boundary, body) = multipart.finish();

        call_method(self.bot, "sendDocument", Some(boundary), body).await
    }
}
