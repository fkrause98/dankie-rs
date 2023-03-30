use serde::Serialize;

use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{bot_command::Scope, BotCommand},
};

/// Gets the list of the bot's commands.
///
/// Represents the [`getMyCommands`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getmycommands
#[derive(Debug, Clone, Serialize)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetMyCommands<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    scope: Scope,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl<'a> GetMyCommands<'a> {
    pub(crate) const fn new(bot: &'a InnerBot) -> Self {
        Self {
            bot,
            scope: Scope::Default,
            language_code: None,
        }
    }

    /// Specifies the scope of the dedicated command list that you want to
    /// retrive. Reflects the `scope` parameter.
    #[allow(clippy::missing_const_for_fn)]
    pub fn scope(mut self, scope: Scope) -> Self {
        self.scope = scope;
        self
    }

    /// Configures the langauge of the dedicated command that you want to
    /// retrieve. Reflects the `language_code` parameter.
    pub fn language_code(mut self, language_code: impl Into<String>) -> Self {
        self.language_code = Some(language_code.into());
        self
    }
}

impl GetMyCommands<'_> {
    /// Calls the method.
    pub async fn call(self) -> Result<Vec<BotCommand>, errors::MethodCall> {
        call_method(
            self.bot,
            "getMyCommands",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
