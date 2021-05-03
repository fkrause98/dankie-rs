use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        file,
        input_file::{InputFile, Photo},
        keyboard,
        message::{self, Message},
        parameters::{ChatId, ImplicitChatId},
    },
    Multipart,
};

/// Sends a photo.
///
/// Reflects the [`sendPhoto`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#sendphoto
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SendPhoto<'a> {
    bot: &'a InnerBot,
    chat_id: ChatId,
    photo: Photo<'a>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<message::Id>,
    allow_sending_without_reply: bool,
    reply_markup: Option<keyboard::Any<'a>>,
}

impl<'a> SendPhoto<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        photo: Photo<'a>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            photo,
            disable_notification: None,
            reply_to_message_id: None,
            allow_sending_without_reply: false,
            reply_markup: None,
        }
    }

    /// Configures whether the message is sent silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }

    /// Configures which message this photo is sent in reply to.
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

impl SendPhoto<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Message, errors::MethodCall> {
        let mut multipart = Multipart::new(7)
            .chat_id("chat_id", &self.chat_id)
            .maybe_str("caption", self.photo.caption.as_deref())
            .maybe_string("parse_mode", self.photo.parse_mode)
            .maybe_string("disabled_notification", self.disable_notification)
            .maybe_string("reply_to_message_id", self.reply_to_message_id)
            .string(
                "allow_sending_without_reply",
                &self.allow_sending_without_reply,
            )
            .maybe_json("reply_markup", self.reply_markup);

        match &self.photo.media {
            InputFile::File {
                filename, bytes, ..
            } => multipart = multipart.file("photo", filename, bytes),
            InputFile::Id(file::Id(photo)) | InputFile::Url(photo) => {
                multipart = multipart.str("photo", photo);
            }
        }

        let (boundary, body) = multipart.finish();

        call_method(self.bot, "sendPhoto", Some(boundary), body).await
    }
}
