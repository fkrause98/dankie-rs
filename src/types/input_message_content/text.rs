#[allow(deprecated)]
use crate::types::parameters::{self, ParseMode, WebPagePreviewState};
use serde::Serialize;

/// Represents an [`InputTextMessageContent`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#inputtextmessagecontent
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
#[must_use]
pub struct Text<'a> {
    message_text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

impl<'a> Text<'a> {
    /// Constructs a new `Text`.
    pub fn new(message_text: impl Into<parameters::Text<'a>>) -> Self {
        let message_text = message_text.into();

        Self {
            message_text: message_text.text,
            parse_mode: message_text.parse_mode,
            disable_web_page_preview: None,
        }
    }

    /// Configures if the web page preview will be shown.
    pub fn is_web_page_preview_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_web_page_preview = Some(is_disabled);
        self
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `is_web_page_preview_disabled` which takes a `bool`"
    )]
    #[allow(deprecated)]
    pub fn web_page_preview(self, state: WebPagePreviewState) -> Self {
        self.is_web_page_preview_disabled(state.is_disabled())
    }
}
