use super::InputFile;
use crate::types::file;
use serde::ser::SerializeMap;

/// Represents a sticker to be sent.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[must_use]
pub struct Sticker<'a> {
    pub(crate) media: InputFile<'a>,
}

impl<'a> Sticker<'a> {
    const fn new(media: InputFile<'a>) -> Self {
        Self { media }
    }

    /// Constructs a `Sticker` from bytes.
    pub fn with_bytes(bytes: &'a [u8]) -> Self {
        Self::new(InputFile::File {
            filename: "sticker.webm",
            bytes,
        })
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_bytes`"
    )]
    pub fn bytes(bytes: &'a [u8]) -> Self {
        Self::with_bytes(bytes)
    }

    /// Constructs a `Sticker` from a file ID.
    ///
    /// # Panics
    ///
    /// Panicks if the ID starts with `attach://`.
    pub fn with_id(id: file::id::Ref<'a>) -> Self {
        assert!(
            !id.0.starts_with("attach://"),
            "\n[tbot] Sticker's ID cannot start with `attach://`\n",
        );

        Self::new(InputFile::Id(id.0))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "use `with_id` which takes a `file::id::Ref<'a>`"
    )]
    pub fn id(id: &'a str) -> Self {
        Self::with_id(file::id::Ref(id))
    }

    /// Constructs a `Sticker` from an URL.
    ///
    /// # Panics
    ///
    /// Panicks if the URL starts with `attach://`.
    pub fn with_url(url: &'a str) -> Self {
        assert!(
            !url.starts_with("attach://"),
            "\n[tbot] Sticker's URL cannot start with `attach://`\n",
        );

        Self::new(InputFile::Url(url))
    }

    #[doc(hidden)]
    #[deprecated(
        since = "0.6.6",
        note = "this method is renamed to `with_url`"
    )]
    pub fn url(url: &'a str) -> Self {
        Self::with_url(url)
    }
}

impl<'a> serde::Serialize for Sticker<'a> {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let mut map = s.serialize_map(None)?;

        map.serialize_entry("type", "sticker")?;
        map.serialize_entry("media", &self.media.with_name("sticker"))?;

        map.end()
    }
}
