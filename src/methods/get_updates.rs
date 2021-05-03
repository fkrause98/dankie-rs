use super::call_method;
use crate::{
    bot::InnerBot,
    errors,
    types::{parameters::AllowedUpdates, update::RawUpdate},
};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
#[must_use]
pub struct GetUpdates<'a> {
    #[serde(skip)]
    bot: &'a InnerBot,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<isize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<AllowedUpdates>,
}

impl<'a> GetUpdates<'a> {
    pub(crate) const fn new(
        bot: &'a InnerBot,
        offset: Option<isize>,
        limit: Option<u8>,
        timeout: Option<u64>,
        allowed_updates: Option<AllowedUpdates>,
    ) -> Self {
        Self {
            bot,
            offset,
            limit,
            timeout,
            allowed_updates,
        }
    }
}

impl GetUpdates<'_> {
    /// Calls the method.
    pub(crate) async fn call(
        self,
    ) -> Result<Vec<RawUpdate>, errors::MethodCall> {
        call_method(
            self.bot,
            "getUpdates",
            None,
            serde_json::to_vec(&self).unwrap(),
        )
        .await
    }
}
