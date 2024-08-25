use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::parameters::{ChatId, ImplicitChatId},
};
use serde::Serialize;

/// Sets a chat's description.
///
/// Reflects the [`setChatDescription`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setchatdescription
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetChatDescription<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    chat_id: ChatId,
    description: String,
}

impl<'a> SetChatDescription<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        chat_id: impl ImplicitChatId,
        description: impl Into<String>,
    ) -> Self {
        Self {
            bot,
            chat_id: chat_id.into(),
            description: description.into(),
        }
    }
}

impl SetChatDescription<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setChatDescription",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
