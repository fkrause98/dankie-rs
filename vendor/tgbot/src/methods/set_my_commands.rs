use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{bot_command::Scope, BotCommand},
};
use serde::Serialize;

/// Sets the list of the bot's commands.
///
/// Represents the [`setMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#setmycommands
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct SetMyCommands<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    commands: Vec<BotCommand>,
    scope: Scope,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> SetMyCommands<'a> {
    pub(crate) fn new(
        bot: &'a InnerBot,
        commands: impl Into<Vec<BotCommand>>,
    ) -> Self {
        Self {
            bot,
            commands: commands.into(),
            scope: Scope::default(),
            language_code: None,
        }
    }

    /// Speicifies the scope for which the list of bot commands is applied.
    /// Reflects the `scope` parameter.
    #[allow(clippy::missing_const_for_fn)]
    pub fn scope(mut self, scope: Scope) -> Self {
        self.scope = scope;
        self
    }

    /// Specifies the language for this list of bot commands.
    /// Reflects the `language_code` parameter.
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.language_code = Some(language_code.into());
        self
    }
}

impl SetMyCommands<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<(), errors::MethodCall> {
        call_method::<bool>(
            self.bot,
            "setMyCommands",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await?;

        Ok(())
    }
}
