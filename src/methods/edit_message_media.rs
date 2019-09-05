use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        input_file::*,
        keyboard::inline,
        message,
        parameters::{ChatId, ImplicitChatId},
    },
};

/// Edits the media of a message sent by the bot itself.
///
/// Reflects the [`editMessageMedia`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditMessageMedia<'a, C> {
    client: &'a Client<C>,
    token: Token,
    chat_id: ChatId<'a>,
    message_id: message::Id,
    media: EditableMedia<'a>,
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a, C> EditMessageMedia<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        message_id: message::Id,
        media: impl Into<EditableMedia<'a>>,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            message_id,
            media: media.into(),
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl<C> IntoFuture for EditMessageMedia<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = types::Message;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        let mut multipart = Multipart::new(5)
            .chat_id("chat_id", self.chat_id)
            .string("message_id", &self.message_id)
            .maybe_json("reply_markup", self.reply_markup);

        match &self.media {
            EditableMedia::Animation(Animation { media, .. })
            | EditableMedia::Audio(Audio { media, .. })
            | EditableMedia::Document(Document { media, .. })
            | EditableMedia::Photo(Photo { media, .. })
            | EditableMedia::Video(Video { media, .. }) => {
                if let InputFile::File { filename, bytes } = media {
                    multipart =
                        multipart.file(self.media.name(), filename, bytes);
                }
            }
        }

        let (boundary, body) = multipart.json("media", self.media).finish();

        Box::new(send_method(
            self.client,
            &self.token,
            "editMessageMedia",
            Some(boundary),
            body,
        ))
    }
}
