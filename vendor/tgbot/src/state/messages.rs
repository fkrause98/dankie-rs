//! A storage of state per message.
//!
//! The [`Messages`] storage can be used to store state for each message
//! separately. For example, when a chatting bot broadcasts a message from
//! Alice to Bob, it can store the ID of the broadcasted message in state
//! for the original message's ID, and if Alice edits her message, the bot
//! can edit the corresponding message on Bob's side.
//!
//! [`Messages`] has a similar API to the API of [`Chats`] — all of the methods
//! from the latter are adapted for the former, except that the key is a special
//! structure [`MessageId`], which contains a message's and its chat's IDs.
//! In addition, for each method working with all records, [`Messages`] provides
//! additional methods with the `in_chat[_by_id]` postfix that work only with
//! messages with the specified chat ID. For example, [`len`] returns the amount
//! of all records, while [`len_in_chat`] returns the amount of records with
//! the specified chat (which is inferred from the context in the case of this
//! method; [`len_in_chat_by_id`], on the other hand, takes a chat ID directly).
//!
//! [`Chats`]: super::Chats
//! [`len`]: Messages::len
//! [`len_in_chat`]: Messages::len_in_chat
//! [`len_in_chat_by_id`]: Messages::len_in_chat_by_id

use crate::{
    contexts::fields::Message,
    types::{chat, message},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::{self, Entry, HashMap, IntoIter},
    iter::FromIterator,
    ops::Index,
};

/// A struct containing a message's and its chat's IDs.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Deserialize)]
#[must_use]
pub struct MessageId {
    /// The ID of the message's chat.
    pub chat_id: chat::Id,
    /// The ID of the message.
    pub message_id: message::Id,
}

impl MessageId {
    /// Constructs a `MessageId` from the provided context.
    pub fn from_context<C>(context: &C) -> Self
    where
        C: Message,
    {
        Self {
            chat_id: context.chat().id,
            message_id: context.message_id(),
        }
    }

    /// Constructs a `MessageId` from the provided message.
    pub const fn from_message(message: &message::Message) -> Self {
        Self {
            chat_id: message.chat.id,
            message_id: message.id,
        }
    }
}

/// A storage of state per message. See [the module's docs] to learn
/// how to use it.
///
/// [the module's docs]: self
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Messages<S> {
    messages: HashMap<MessageId, S>,
}

impl<S> Messages<S> {
    /// Constructs a new message storage.
    #[must_use]
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    /// Constructs a new message storage with capacity for `n` messages.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            messages: HashMap::with_capacity(capacity),
        }
    }

    /// Returns an iterator over the stored messages.
    pub fn all_messages(&self) -> impl Iterator<Item = MessageId> + '_ {
        self.messages.keys().copied()
    }

    /// Returns an iterator over stored message IDs for a chat by its ID.
    pub fn messages_in_chat_by_id(
        &self,
        chat_id: chat::Id,
    ) -> impl Iterator<Item = message::Id> + '_ {
        self.all_messages().filter_map(move |id| {
            if id.chat_id == chat_id {
                Some(id.message_id)
            } else {
                None
            }
        })
    }

    /// Returns an iterator over stored message IDs for a chat, which ID is
    /// inferred from the context.
    pub fn messages_in_chat<C>(
        &self,
        context: &C,
    ) -> impl Iterator<Item = message::Id> + '_
    where
        C: Message,
    {
        self.messages_in_chat_by_id(context.chat().id)
    }

    /// Returns an iterator over the stored states.
    pub fn all_states(&self) -> impl Iterator<Item = &S> {
        self.messages.values()
    }

    /// Returns an iterator over stored states for messages in a chat
    /// by its ID.
    pub fn states_in_chat_by_id(
        &self,
        chat_id: chat::Id,
    ) -> impl Iterator<Item = &S> {
        self.iter_in_chat_by_id(chat_id).map(|(_, state)| state)
    }

    /// Returns an iterator over stored states for messages in a chat, which ID
    /// is inferred from the context.
    pub fn states_in_chat<C>(&self, context: &C) -> impl Iterator<Item = &S>
    where
        C: Message,
    {
        self.states_in_chat_by_id(context.chat().id)
    }

    /// Returns an iterator over stored messages and their states.
    pub fn iter(&self) -> impl Iterator<Item = (MessageId, &S)> {
        Iter(self.messages.iter())
    }

    /// Returns an iterator over stored messages and their states in a chat
    /// by its ID.
    pub fn iter_in_chat_by_id(
        &self,
        chat_id: chat::Id,
    ) -> impl Iterator<Item = (message::Id, &S)> {
        self.iter().filter_map(move |(id, state)| {
            if id.chat_id == chat_id {
                Some((id.message_id, state))
            } else {
                None
            }
        })
    }

    /// Returns an iterator over stored messages and their states in a chat,
    /// which ID is inferred from the context.
    pub fn iter_in_chat<C>(
        &self,
        context: &C,
    ) -> impl Iterator<Item = (message::Id, &S)>
    where
        C: Message,
    {
        self.iter_in_chat_by_id(context.chat().id)
    }

    /// Returns a mutable iterator over the stored messages and their states.
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (MessageId, &mut S)> {
        IterMut(self.messages.iter_mut())
    }

    /// Returns a mutable iterator over the stored messages and their states
    /// in a chat by its ID.
    pub fn iter_mut_in_chat_by_id(
        &mut self,
        chat_id: chat::Id,
    ) -> impl Iterator<Item = (message::Id, &mut S)> {
        self.iter_mut().filter_map(move |(id, state)| {
            if id.chat_id == chat_id {
                Some((id.message_id, state))
            } else {
                None
            }
        })
    }

    /// Returns a mutable iterator over the stored messages and their states
    /// in a chat, which ID is inferred from the context.
    pub fn iter_mut_in_chat<C>(
        &mut self,
        context: &C,
    ) -> impl Iterator<Item = (message::Id, &mut S)>
    where
        C: Message,
    {
        self.iter_mut_in_chat_by_id(context.chat().id)
    }

    /// Returns an owning iterator over the stored messages and their states
    /// in a chat by its ID.
    pub fn into_iter_in_chat_by_id(
        self,
        chat_id: chat::Id,
    ) -> impl Iterator<Item = (message::Id, S)> {
        self.into_iter().filter_map(move |(id, state)| {
            if id.chat_id == chat_id {
                Some((id.message_id, state))
            } else {
                None
            }
        })
    }

    /// Returns an owning iterator over the stored messages and their states
    /// in a chat, which ID is inferred from the context.
    pub fn into_iter_in_chat<C>(
        self,
        context: &C,
    ) -> impl Iterator<Item = (message::Id, S)>
    where
        C: Message,
    {
        self.into_iter_in_chat_by_id(context.chat().id)
    }

    /// Returns how many messages are stored.
    #[must_use]
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Returns how many messages from a chat are stored.
    #[must_use]
    pub fn len_in_chat_by_id(&self, chat_id: chat::Id) -> usize {
        self.iter_in_chat_by_id(chat_id).count()
    }

    /// Returns how many messages from a chat, which ID is inferred
    /// from the context, are stored.
    #[must_use]
    pub fn len_in_chat<C>(&self, context: &C) -> usize
    where
        C: Message,
    {
        self.iter_in_chat(context).count()
    }

    /// Returns the storage's capacity.
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.messages.capacity()
    }

    /// Returns `true` if the store is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Returns if the store does *not* have messages from a chat.
    #[must_use]
    pub fn is_empty_in_chat_by_id(&self, chat_id: chat::Id) -> bool {
        self.iter_in_chat_by_id(chat_id).next().is_none()
    }

    /// Returns if the store does *not* have messages from a chat, inferring
    /// its ID from the context.
    #[must_use]
    pub fn is_empty_in_chat<C>(&self, context: &C) -> bool
    where
        C: Message,
    {
        self.is_empty_in_chat_by_id(context.chat().id)
    }

    /// Clears the storage, returning each stored item in an iterator.
    #[must_use = "use `clear` if you don't need the iterator"]
    pub fn drain(&mut self) -> impl Iterator<Item = (MessageId, S)> + '_ {
        self.messages.drain()
    }

    /// Clears the store.
    pub fn clear(&mut self) {
        self.messages.clear();
    }

    /// Deletes state for all messages from a chat.
    pub fn clear_in_chat_by_id(&mut self, chat_id: chat::Id) {
        self.retain(|id, _| id.chat_id != chat_id);
    }

    /// Deletes state for all messages from a chat, inferring its ID
    /// from the context.
    pub fn clear_in_chat<C>(&mut self, context: &C)
    where
        C: Message,
    {
        self.clear_in_chat_by_id(context.chat().id);
    }

    /// Reserves capacity for `n` additional messages.
    pub fn reserve(&mut self, additional: usize) {
        self.messages.reserve(additional);
    }

    /// Shrinks the capacity to already stored messages.
    pub fn shrink_to_fit(&mut self) {
        self.messages.shrink_to_fit();
    }

    /// Gets a message's state by its ID.
    #[must_use]
    pub fn get_by_id(&self, id: MessageId) -> Option<&S> {
        self.messages.get(&id)
    }

    /// Gets a message's state, inferring its ID from the context.
    #[must_use]
    pub fn get<C>(&self, context: &C) -> Option<&S>
    where
        C: Message,
    {
        self.get_by_id(MessageId::from_context(context))
    }

    /// Gets a mutable reference to a message's state by its ID.
    #[must_use]
    pub fn get_mut_by_id(&mut self, id: MessageId) -> Option<&mut S> {
        self.messages.get_mut(&id)
    }

    /// Gets a mutable reference to a message's state, inferring its ID
    /// from the context.
    #[must_use]
    pub fn get_mut<C>(&mut self, context: &C) -> Option<&mut S>
    where
        C: Message,
    {
        self.get_mut_by_id(MessageId::from_context(context))
    }

    /// Gets an entry to a message's state by its ID.
    #[must_use]
    pub fn entry_by_id(&mut self, id: MessageId) -> Entry<MessageId, S> {
        self.messages.entry(id)
    }

    /// Gets an entry to a message's state, inferring its ID from the context.
    #[must_use]
    pub fn entry<C>(&mut self, context: &C) -> Entry<MessageId, S>
    where
        C: Message,
    {
        self.entry_by_id(MessageId::from_context(context))
    }

    /// Checks if there's state for a message by its ID.
    #[must_use]
    pub fn has_by_id(&self, id: MessageId) -> bool {
        self.messages.contains_key(&id)
    }

    /// Checks if there's state for a message, inferring its ID
    /// from the context.
    #[must_use]
    pub fn has<C>(&self, context: &C) -> bool
    where
        C: Message,
    {
        self.has_by_id(MessageId::from_context(context))
    }

    /// Inserts state for a message by its ID. Returns the previous state.
    pub fn insert_by_id(&mut self, id: MessageId, value: S) -> Option<S> {
        self.messages.insert(id, value)
    }

    /// Inserts state for a message, inferring its ID from the context.
    /// Returns the previous state.
    pub fn insert<C>(&mut self, context: &C, value: S) -> Option<S>
    where
        C: Message,
    {
        self.insert_by_id(MessageId::from_context(context), value)
    }

    /// Removes and returns a message's state by its ID.
    pub fn remove_by_id(&mut self, id: MessageId) -> Option<S> {
        self.messages.remove(&id)
    }

    /// Removes and returns a message's state, inferring its ID
    /// from the context.
    pub fn remove<C>(&mut self, context: &C) -> Option<S>
    where
        C: Message,
    {
        self.remove_by_id(MessageId::from_context(context))
    }

    /// Calls the predicate for each stored entry and deletes entries for which
    /// the predicate returns `false`.
    pub fn retain<P>(&mut self, mut predicate: P)
    where
        P: FnMut(MessageId, &mut S) -> bool,
    {
        self.messages.retain(|&id, state| predicate(id, state));
    }
}

impl<S> IntoIterator for Messages<S> {
    type Item = (MessageId, S);
    type IntoIter = IntoIter<MessageId, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.messages.into_iter()
    }
}

/// An iterator over the entries of [`Messages`].
pub struct Iter<'a, S>(hash_map::Iter<'a, MessageId, S>);

impl<'a, S> Iterator for Iter<'a, S> {
    type Item = (MessageId, &'a S);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(&id, state)| (id, state))
    }
}

impl<'a, S> IntoIterator for &'a Messages<S> {
    type Item = (MessageId, &'a S);
    type IntoIter = Iter<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        Iter(self.messages.iter())
    }
}

/// A mutable iterator over the entries of [`Messages`].
pub struct IterMut<'a, S>(hash_map::IterMut<'a, MessageId, S>);

impl<'a, S> Iterator for IterMut<'a, S> {
    type Item = (MessageId, &'a mut S);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(&id, state)| (id, state))
    }
}

impl<'a, S> IntoIterator for &'a mut Messages<S> {
    type Item = (MessageId, &'a mut S);
    type IntoIter = IterMut<'a, S>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut(self.messages.iter_mut())
    }
}

impl<S> Extend<(MessageId, S)> for Messages<S> {
    fn extend<I: IntoIterator<Item = (MessageId, S)>>(&mut self, iterator: I) {
        self.messages.extend(iterator);
    }
}

impl<'a, S: Copy> Extend<(MessageId, &'a S)> for Messages<S> {
    fn extend<I: IntoIterator<Item = (MessageId, &'a S)>>(
        &mut self,
        iterator: I,
    ) {
        self.extend(iterator.into_iter().map(|(id, &state)| (id, state)));
    }
}

impl<S> FromIterator<(MessageId, S)> for Messages<S> {
    fn from_iter<I: IntoIterator<Item = (MessageId, S)>>(iter: I) -> Self {
        Self {
            messages: HashMap::from_iter(iter),
        }
    }
}

impl<S> Index<MessageId> for Messages<S> {
    type Output = S;

    fn index(&self, id: MessageId) -> &S {
        self.messages.index(&id)
    }
}

impl<S> Default for Messages<S> {
    fn default() -> Self {
        Self::new()
    }
}
