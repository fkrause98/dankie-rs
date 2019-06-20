//! Types related to downloadable files.

use serde::*;

pub mod id;

pub use id::Id;

/// Represents a [`File`].
///
/// [`File`]: https://core.telegram.org/bots/api#file
#[derive(Debug, PartialEq, Eq, Clone, Hash, Deserialize)]
// todo: #[non_exhaustive]
pub struct File {
    /// The ID of the file.
    #[serde(rename = "file_id")]
    pub id: Id,
    /// The size fo the file.
    #[serde(rename = "file_size")]
    pub size: Option<u32>,
    /// The path of the file.
    #[serde(rename = "file_path")]
    pub path: Option<String>,
}
