use super::ChatMethods;
use crate::{methods::ForwardMessage, types::parameters::ImplicitChatId};

/// Provides methods for forwardable messages.
pub trait Forwardable<'b, C: 'static>: ChatMethods<'b, C> {
    /// Forwards this message to another chat.
    fn forward_to<'a>(
        &'a self,
        chat_id: impl ImplicitChatId<'a>,
    ) -> ForwardMessage<'a, C> {
        self.bot()
            .forward_message(chat_id, self.chat().id, self.message_id())
    }
}
