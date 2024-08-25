//! Types related to Telegram Passport.

mod credentials;
mod data;
pub mod element;
mod file;

pub use {credentials::Credentials, data::Data, element::Element, file::File};
