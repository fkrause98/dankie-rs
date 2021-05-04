use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{keyboard::inline, InlineMessageId},
};
use serde::Serialize;

/// Edits a live location sent via the inline mode.
///
/// Reflects the [`editMessageLiveLocation`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#editmessagelivelocation
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct EditInlineLocation<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    inline_message_id: InlineMessageId,
    latitude: f64,
    longitude: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<inline::Keyboard>,
}

impl<'a> EditInlineLocation<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        inline_message_id: InlineMessageId,
        (latitude, longitude): (f64, f64),
    ) -> Self {
        Self {
            bot,
            inline_message_id,
            latitude,
            longitude,
            reply_markup: None,
        }
    }

    /// Configures an inline keyboard for the message.
    /// Reflects the `reply_markup` parameter.
    #[allow(clippy::missing_const_for_fn)]
    pub fn reply_markup(mut self, markup: inline::Keyboard) -> Self {
        self.reply_markup = Some(markup);
        self
    }
}

impl EditInlineLocation<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "editMessageLiveLocation",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
