use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        inline_message_id::InlineMessageId,
        input_file::{
            Animation, Audio, Document, EditableMedia, InputFile, Photo, Video,
        },
        keyboard::inline,
    },
    Multipart,
};

/// Edits the media of a message sent via the inline mode.
///
/// Reflects the [`editMessageMedia`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagemedia
#[derive(Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineMedia<'a> {
    bot: &'a InnerBot,
    inline_message_id: InlineMessageId,
    media: EditableMedia,
    reply_markup: Option<inline::Keyboard<'a>>,
}

impl<'a> EditInlineMedia<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        inline_message_id: InlineMessageId,
        media: impl Into<EditableMedia>,
    ) -> Self {
        Self {
            bot,
            inline_message_id,
            media: media.into(),
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    pub const fn reply_markup(mut self, markup: inline::Keyboard<'a>) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditInlineMedia<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        let mut multipart = Multipart::new(4)
            .str("inline_message_id", &self.inline_message_id.0)
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

        let (boundary, body) = multipart.json("media", &self.media).finish();

        call_method::<bool>(self.bot, "editMessageMedia", Some(boundary), body)
            .await?;

        Ok(())
    }
}
