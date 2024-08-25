use serde::ser::SerializeMap;

/// Represents a [`ForceReply`].
///
/// [`ForceReply`]: https://core.telegram.org/bots/api#forcereply
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Default)]
#[must_use]
pub struct ForceReply {
    // force_reply is added when serialized
    selective: Option<bool>,
}

impl ForceReply {
    /// Constructs a new `ForceReply`.
    pub const fn new() -> Self {
        Self { selective: None }
    }

    /// Configure `selective`.
    pub const fn is_selective(mut self, is_selective: bool) -> Self {
        self.selective = Some(is_selective);
        self
    }
}

impl serde::Serialize for ForceReply {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        let len = if self.selective.is_some() { 2 } else { 1 };

        let mut map = s.serialize_map(Some(len))?;

        map.serialize_entry("force_reply", &true)?;

        if let Some(selective) = self.selective {
            map.serialize_entry("selective", &selective)?;
        }

        map.end()
    }
}
