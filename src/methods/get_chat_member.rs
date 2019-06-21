use super::*;
use crate::{
    errors,
    internal::{BoxFuture, Client},
    types::{
        chat,
        parameters::{ChatId, ImplicitChatId},
        user,
    },
};

/// Represents the [`getChatMember`][docs] method.
///
/// [docs]: https://core.telegram.org/bots/api#getchatmember
#[derive(Serialize, Debug, Clone)]
#[must_use = "methods do nothing unless turned into a future"]
pub struct GetChatMember<'a, C> {
    #[serde(skip)]
    client: &'a Client<C>,
    #[serde(skip)]
    token: Token,
    chat_id: ChatId<'a>,
    user_id: user::Id,
}

impl<'a, C> GetChatMember<'a, C> {
    pub(crate) fn new(
        client: &'a Client<C>,
        token: Token,
        chat_id: impl ImplicitChatId<'a>,
        user_id: user::Id,
    ) -> Self {
        Self {
            client,
            token,
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

impl<C> IntoFuture for GetChatMember<'_, C>
where
    C: hyper::client::connect::Connect + Sync + 'static,
    C::Transport: 'static,
    C::Future: 'static,
{
    type Future = BoxFuture<Self::Item, Self::Error>;
    type Item = chat::Member;
    type Error = errors::MethodCall;

    fn into_future(self) -> Self::Future {
        Box::new(send_method(
            self.client,
            &self.token,
            "getChatMember",
            None,
            serde_json::to_vec(&self).unwrap(),
        ))
    }
}
