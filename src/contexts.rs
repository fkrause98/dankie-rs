//! Contexts for update handlers.
//!
//! A context is a struct that is passed to update handlers, contains data about
//! the update, and provides methods that infer certain data from the update.
//! For example:
//!
//! ```no_run
//! # async fn foo() {
//! use tbot::prelude::*;
//!
//! let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
//!
//! bot.text(|context| async move {
//!     let reversed: String = context.text.value.chars().rev().collect();
//!     context.send_message_in_reply(&reversed).call().await.unwrap();
//! });
//! # }
//! ```
//!
//! Here, we set a [`text`][text-handler] handler for our bot. Whenever we get
//! a text message, the handler is called with a reference to
//! a [`Text`][text-context] context that contains data about the incoming data,
//! e.g. the text of the message. Then we call the [`send_message_in_reply`]
//! method on the context, which does what its name says: sends a message
//! in the same chat in reply to the incoming message, inferring your bot's
//! token and IDs of the chat and the message.
//!
//! All contexts have one common field named `bot`. Through this field, you can
//! call any method using a [`Bot`]:
//!
//! ```no_run
//! # async fn foo() {
//! # use tbot::prelude::*;
//! # let mut bot = tbot::Bot::new(String::new()).event_loop();
//! use tbot::types::chat;
//! const ADMIN_CHAT: chat::Id = chat::Id(0);
//!
//! bot.text(|context| async move {
//!     context
//!         .bot
//!         .send_message(ADMIN_CHAT, "New message!")
//!         .call()
//!         .await
//!         .unwrap();
//! });
//! # }
//! ```
//!
//! Most contexts implement certain traits, such as [`ChatMethods`]
//! or [`Pinnable`]. These traits share common methods between contexts,
//! e.g. [`send_message_in_reply`] you have seen above.
//!
//! [text-handler]: ../bot/struct.Bot.html#method.text
//! [text-context]: ./struct.Text.html
//! [`send_message_in_reply`]: ./traits/trait.ChatMethods.html#method.send_message_in_reply
//! [`Bot`]: ../bot/struct.Bot.html
//! [`ChatMethods`]: ./traits/trait.ChatMethods.html
//! [`Pinnable`]: ./traits/trait.Pinnable.html

#[macro_use]
mod macros;

mod animation;
mod any_update;
mod audio;
mod chosen_inline;
mod command;
mod connected_website;
mod contact;
mod created_group;
mod data_callback;
mod deleted_chat_photo;
mod dice;
mod document;
mod edited_animation;
mod edited_audio;
mod edited_command;
mod edited_document;
mod edited_location;
mod edited_photo;
mod edited_text;
mod edited_video;
mod game;
mod game_callback;
mod inline;
mod invoice;
mod left_member;
mod location;
mod migration;
mod new_chat_photo;
mod new_chat_title;
mod new_members;
mod passport;
mod payment;
mod photo;
mod pinned_message;
mod poll;
mod poll_answer;
mod pre_checkout;
mod shipping;
mod sticker;
mod text;
mod unhandled;
mod updated_poll;
mod venue;
mod video;
mod video_note;
mod voice;

pub mod fields;
pub mod methods;
mod proximity_alert;

pub use animation::Animation;
pub use any_update::AnyUpdate;
pub use audio::Audio;
pub use chosen_inline::ChosenInline;
pub use command::Command;
pub use connected_website::ConnectedWebsite;
pub use contact::Contact;
pub use created_group::CreatedGroup;
pub use data_callback::{InlineDataCallback, MessageDataCallback};
pub use deleted_chat_photo::DeletedChatPhoto;
pub use dice::Dice;
pub use document::Document;
pub use edited_animation::EditedAnimation;
pub use edited_audio::EditedAudio;
pub use edited_command::EditedCommand;
pub use edited_document::EditedDocument;
pub use edited_location::EditedLocation;
pub use edited_photo::EditedPhoto;
pub use edited_text::EditedText;
pub use edited_video::EditedVideo;
pub use game::Game;
pub use game_callback::{InlineGameCallback, MessageGameCallback};
pub use inline::Inline;
pub use invoice::Invoice;
pub use left_member::LeftMember;
pub use location::Location;
pub use migration::Migration;
pub use new_chat_photo::NewChatPhoto;
pub use new_chat_title::NewChatTitle;
pub use new_members::NewMembers;
pub use passport::Passport;
pub use payment::Payment;
pub use photo::Photo;
pub use pinned_message::PinnedMessage;
pub use poll::Poll;
pub use poll_answer::PollAnswer;
pub use pre_checkout::PreCheckout;
pub use proximity_alert::ProximityAlert;
pub use shipping::Shipping;
pub use sticker::Sticker;
pub use text::Text;
pub use unhandled::Unhandled;
pub use updated_poll::UpdatedPoll;
pub use venue::Venue;
pub use video::Video;
pub use video_note::VideoNote;
pub use voice::Voice;
