use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{
        parameters::{ChatId, ImplicitChatId},
        Chat,
    },
};
use serde::Serialize;

/// Gets information about a chat.
///
/// Reflects the [`getChat`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchat
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChat<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
}

impl<'a> GetChat<'a> {
    pub(crate) fn new(bot: &'a InnerBot, chat_id: impl ImplicitChatId) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }
}

impl GetChat<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Chat, errors::MethodCall> {
        call_method(
            self.bot,
            "getChat",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
