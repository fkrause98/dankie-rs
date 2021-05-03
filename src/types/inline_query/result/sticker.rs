use crate::types::{file, InputMessageContent};
use serde::Serialize;

/// Represents an [`InlineQueryResultCachedSticker`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inlinequeryresultcachedsticker
#[derive(Debug, PartialEq, Clone, Serialize)]
#[must_use]
pub struct Sticker<'a> {
    #[serde(rename = "sticker_file_id")]
    id: file::Id<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent<'a>>,
}

impl<'a> Sticker<'a> {
    /// Constructs a `Sticker`.
    pub const fn new(id: file::Id<'a>) -> Self {
        Self {
            id,
            input_message_content: None,
        }
    }

    /// Configures the content shown after sending the message.
    pub fn input_message_content(
        mut self,
        content: impl Into<InputMessageContent<'a>>,
    ) -> Self {
        self.input_message_content = Some(content.into());
        self
    }
}
