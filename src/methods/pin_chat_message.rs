use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        message,
        parameters::{ChatId, ImplicitChatId},
    },
};
use serde::Serialize;

/// Pins a message in a chat.
///
/// Reflects the [`pinChatMessage`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#pinchatmessage
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct PinChatMessage<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    message_id: message::Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
}

impl<'a> PinChatMessage<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        message_id: message::Id,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    /// Configures whether the message is pinned silently.
    /// Reflects the `disable_notification` parameter.
    pub const fn is_notification_disabled(mut self, is_disabled: bool) -> Self {
        self.disable_notification = Some(is_disabled);
        self
    }
}

impl PinChatMessage<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "pinChatMessage",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
