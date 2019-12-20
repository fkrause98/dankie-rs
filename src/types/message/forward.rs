//! Types representing a forward.

use super::Id;
use crate::types::{Chat, User};

/// Represents a forward source.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
#[must_use]
pub enum From {
    /// The forward is from a user.
    User(User),
    /// The forward is from a user who decided to hide their profile.
    HiddenUser(String),
    /// The forward is from a channel.
    #[non_exhaustive]
    Channel {
        /// Information about the channel.
        chat: Box<Chat>,
        /// The ID of the original message.
        message_id: Id,
        /// The author's signature.
        signature: Option<String>,
    },
}

/// Represents forward information.
#[derive(Debug, PartialEq, Clone)]
#[non_exhaustive]
#[must_use]
pub struct Forward {
    /// The author of the original message.
    pub from: From,
    /// The timestamp of the original message.
    pub date: i64,
}

impl From {
    /// Checks if `self` is `User`.
    #[must_use]
    pub fn is_user(&self) -> bool {
        match self {
            Self::User(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Hidden`.
    #[must_use]
    pub fn is_hidden_user(&self) -> bool {
        match self {
            Self::HiddenUser(..) => true,
            _ => false,
        }
    }

    /// Checks if `self` is `Channel`.
    #[must_use]
    pub fn is_channel(&self) -> bool {
        match self {
            Self::Channel { .. } => true,
            _ => false,
        }
    }
}
