use is_macro::Is;
use serde::{ser::SerializeMap, Serialize};

use crate::types::{
    parameters::{ChatId, ImplicitChatId},
    user,
};

/// A scope for which a list of bot commands is applied.
///
/// Represents [`BotCommandScope`].
///
/// [`BotCommandScope`]: https://core.telegram.org/bots/api#botcommandscope
#[derive(Debug, PartialEq, Eq, Clone, Hash, Default, Is)]
#[non_exhaustive]
#[must_use]
pub enum Scope {
    /// The default scope, used when no narrower scope is specified.
    #[default]
    Default,
    /// A scope for all private chats.
    AllPrivateChats,
    /// A scope for all group chats.
    AllGroupChats,
    /// A scope for administrators of all chats.
    AllChatAdministrators,
    /// A scope for a particular chat.
    Chat(ChatId),
    /// A scope for administrators of a particular chat.
    ChatAdministrators(ChatId),
    /// A scope for a particular member of a specific chat.
    ChatMember(ChatId, user::Id),
}

impl Scope {
    /// Constructs a scope for all private chats.
    pub const fn with_all_private_chats() -> Self {
        Self::AllPrivateChats
    }

    /// Constructs a scope for all group chats.
    pub const fn with_all_group_chats() -> Self {
        Self::AllGroupChats
    }

    /// Construct a scope for administrators of all chats.
    pub const fn with_all_chat_administrators() -> Self {
        Self::AllChatAdministrators
    }

    /// Constructs a scope for a particular chat.
    pub fn with_chat(chat_id: impl ImplicitChatId) -> Self {
        Self::Chat(chat_id.into())
    }

    /// Constructs a scope for administrators of a particular chat.
    pub fn with_chat_administrators(chat_id: impl ImplicitChatId) -> Self {
        Self::ChatAdministrators(chat_id.into())
    }

    /// Constructs a scope for a particular member of the given chat.
    pub fn with_chat_member(
        chat_id: impl ImplicitChatId,
        user_id: user::Id,
    ) -> Self {
        Self::ChatMember(chat_id.into(), user_id)
    }
}

impl Serialize for Scope {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Default => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "default")?;
                map.end()
            }
            Self::AllPrivateChats => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "all_private_chats")?;
                map.end()
            }
            Self::AllGroupChats => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "all_group_chats")?;
                map.end()
            }
            Self::AllChatAdministrators => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "all_chat_administrators")?;
                map.end()
            }
            Self::Chat(chat_id) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "chat")?;
                map.serialize_entry("chat_id", chat_id)?;
                map.end()
            }
            Self::ChatAdministrators(chat_id) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "chat_administrators")?;
                map.serialize_entry("chat_id", chat_id)?;
                map.end()
            }
            Self::ChatMember(chat_id, user_id) => {
                let mut map = serializer.serialize_map(Some(3))?;
                map.serialize_entry("type", "chat_member")?;
                map.serialize_entry("chat_id", chat_id)?;
                map.serialize_entry("user_id", user_id)?;
                map.end()
            }
        }
    }
}
