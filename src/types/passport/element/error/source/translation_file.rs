//! Types related to translation file errors.

use is_macro::Is;
use serde::Serialize;
use std::borrow::Cow;

/// Represents possible element kinds for translation error.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
#[must_use]
pub enum Kind {
    /// An error in the user's passport.
    Passport,
    /// An error in the user's driver license.
    DriverLicense,
    /// An error in the user's identity card.
    IdentityCard,
    /// An error in the user's internal passport.
    InternalPassport,
    /// An error in the user's utility bill.
    UtilityBill,
    /// An error in the user's bank statement.
    BankStatement,
    /// An error in the user's rental agreement.
    RentalAgreement,
    /// An error in the user's passport registration.
    PassportRegistration,
    /// An error in the user's temporary registration.
    TemporaryRegistration,
}

/// Represents a [`PassportElementErrorTranslationFile`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrortranslationfile
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct TranslationFile<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hash: Cow<'a, str>,
}

impl<'a> TranslationFile<'a> {
    /// Constructs a new `TranslationFile`.
    pub fn new(kind: Kind, file_hash: impl Into<Cow<'a, str>>) -> Self {
        Self {
            kind,
            file_hash: file_hash.into(),
        }
    }
}

/// Represents a [`PassportElementErrorTranslationFiles`][docs].
///
/// [docs]: https://core.telegram.org/bots/api#passportelementerrortranslationfiles
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
#[must_use]
pub struct TranslationFiles<'a> {
    #[serde(rename = "type")]
    kind: Kind,
    file_hashes: Vec<Cow<'a, str>>,
}

impl<'a> TranslationFiles<'a> {
    /// Constructs new `TranslationFiles`.
    pub fn new<F>(kind: Kind, file_hashes: F) -> Self
    where
        F: IntoIterator,
        F::Item: Into<Cow<'a, str>>,
    {
        Self {
            kind,
            file_hashes: file_hashes.into_iter().map(Into::into).collect(),
        }
    }
}
