//! Telegram Bots API methods in form of structs.
//!
//! Note that structs here are somewhat raw, because they require tokens when
//! construcing. You would more like using construction methods on `Tbot` that
//! infer `token`, and methods on context structs that in addition infer things
//! like `chat_id` or `callback_query_id`.
//!
//! Though methods don't implement a trait like `Method` for reasons, they all
//! have methods `new` with varying number of parameters and `into_future` that
//! returns a `Future` which resolves with either Telegram's response or an
//! error. The design philosophy is that all required parameters are passed
//! through `new` and all optional parameters are set with a dedicated method.
//! Then `into_future` is called to get the `Future`, handling is done and the
//! future is passed to a runner. For convenience, we re-export
//! `tokio::{run, spawn}` as `tbot::{run, spawn}`.
//!
//! For example, here's how you'd call `getMe`:
//!
//! ```
//! # use tbot::prelude::*;
//! let request = tbot::methods::GetMe::new(env!("BOT_TOKEN"))
//!     .into_future()
//!     .map_err(|_| ())
//!     .map(|me| println!("Here I am: {:#?}", me));
//!
//! tbot::run(request);
//! ```

use super::*;

mod delete_webhook;
mod edit_inline_location;
mod edit_message_location;
mod forward_message;
mod get_me;
mod get_updates;
mod get_user_profile_photos;
mod get_webhook_info;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_document;
mod send_location;
mod send_media_group;
mod send_message;
mod send_photo;
mod send_venue;
mod send_video;
mod send_video_note;
mod send_voice;
mod set_webhook;
mod stop_inline_location;
mod stop_message_location;

pub use self::{
    edit_inline_location::*, edit_message_location::*, forward_message::*,
    get_me::*, get_user_profile_photos::*, get_webhook_info::*,
    send_animation::*, send_audio::*, send_chat_action::*, send_contact::*,
    send_document::*, send_location::*, send_media_group::*, send_message::*,
    send_photo::*, send_venue::*, send_video::*, send_video_note::*,
    send_voice::*, stop_inline_location::*, stop_message_location::*,
};

pub(crate) use self::{delete_webhook::*, get_updates::*, set_webhook::*};

mod methods_trait;
pub use self::methods_trait::*;

mod call_method;
use self::call_method::*;

/// An error happened during request. Different errors may happen, so this is
/// an enum representing error that may happen during request.
#[derive(Debug)]
pub enum DeliveryError {
    /// Telegram Bots API responded with an HTML page what usually means it's
    /// down.
    TelegramOutOfService,
    /// `serde_json` couldn't parse the response. Most probably, it's a bug in
    /// `tbot` that tried to parse the response into a wrong struct, what you
    /// should fill an issue for on [our GitLab repository][issues].
    ///
    /// [issues]: https://gitlab.com/snejugal/tbot/issues
    InvalidResponse(serde_json::error::Error),
    /// Some error happened during sending the request.
    NetworkError(hyper::Error),
    /// Telegram returned an error in response. That is most probably your
    /// fault.
    RequestError {
        /// Human-readable description of the error.
        description: String,
        /// Error code reflected through HTTP error codes (for example, 401).
        error_code: i64,
        /// The group moved to a supergroup.
        migrate_to_chat_id: Option<i64>,
        /// When exceeding flood control, you must wait for this amount of
        /// seconds before making another request.
        retry_after: Option<u64>,
    },
}

#[cfg(feature = "proxy")]
/// Provides the proxy method.
pub trait ProxyMethod {
    /// Configures the proxy the method will be sent via.
    fn proxy(self, proxy: proxy::Proxy) -> Self;
}
