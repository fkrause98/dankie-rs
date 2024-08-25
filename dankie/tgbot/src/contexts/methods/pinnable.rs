use super::Message;
use crate::methods::PinChatMessage;

/// Provides methods for pinnable messages.
pub trait Pinnable: Message {
    /// Pins this message.
    fn pin_this_message(&self) -> PinChatMessage<'_> {
        self.bot()
            .pin_chat_message(self.chat().id, self.message_id())
    }
}
