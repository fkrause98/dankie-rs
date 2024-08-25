use super::FormattingState;
use crate::{
    markup::{
        self, bold, code_block, html, inline_code, italic, link, markdown_v2,
        mention, strikethrough, underline,
    },
    types::User,
};
use is_macro::Is;

/// Represents a string with formatting options.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct FormattedText {
    /// The text.
    pub value: String,
    /// `true` if bold is applied to this string.
    pub is_bold: bool,
    /// `true` if italic is applied to this string.
    pub is_italic: bool,
    /// `true` if strikethrough is applied to this string.
    pub is_strikethrough: bool,
    /// `true` if underline is applied to this string.
    pub is_underline: bool,
}

impl FormattedText {
    pub(crate) const fn plain(value: String) -> Self {
        Self {
            value,
            is_bold: false,
            is_italic: false,
            is_strikethrough: false,
            is_underline: false,
        }
    }

    pub(crate) const fn from_state(
        value: String,
        state: &FormattingState,
    ) -> Self {
        Self {
            value,
            is_bold: state.is_bold,
            is_italic: state.is_italic,
            is_strikethrough: state.is_strikethrough,
            is_underline: state.id_underline,
        }
    }
}

/// Represents the semantic meaning of the entity.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
pub enum Kind {
    /// A mention.
    Mention,
    /// A hashtag.
    Hashtag,
    /// A cashtag (e.g. `$TBOT`).
    Cashtag,
    /// A bot command.
    BotCommand,
    /// An URL.
    Url,
    /// An email.
    Email,
    /// A phone number.
    PhoneNumber,
    /// A clickable text link.
    TextLink(String),
    /// A mention for users without username.
    TextMention(User),
}

/// Represents a semantic entity.
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct SemanticEntity {
    /// The semantic meaning.
    pub kind: Option<Kind>,
    /// A `Vec` of formatted strings.
    pub value: Vec<FormattedText>,
}

/// Represents a parsed entity.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Is)]
pub enum Entity {
    /// Inline code.
    Code(String),
    /// A code block.
    Pre {
        /// The code's programming language.
        language: Option<String>,
        /// The code.
        value: String,
    },
    /// Text that may have semantic meaning.
    Semantic(SemanticEntity),
}

fn to_formattable<'a>(
    formatted: &'a [FormattedText],
) -> Vec<Box<dyn markup::Formattable + 'a>> {
    formatted
        .iter()
        .map(
            |FormattedText {
                 value,
                 is_bold,
                 is_italic,
                 is_underline,
                 is_strikethrough,
             }| {
                let mut formatted: Box<dyn markup::Formattable> =
                    Box::new(value.as_str());

                if *is_bold {
                    formatted = Box::new(bold(formatted));
                }

                if *is_italic {
                    formatted = Box::new(italic(formatted));
                }

                if *is_underline {
                    formatted = Box::new(underline(formatted));
                }

                if *is_strikethrough {
                    formatted = Box::new(strikethrough(formatted));
                }

                formatted
            },
        )
        .collect()
}

impl markdown_v2::Formattable for SemanticEntity {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match &self.kind {
            None
            | Some(
                Kind::Mention
                | Kind::Hashtag
                | Kind::Cashtag
                | Kind::BotCommand
                | Kind::Url
                | Kind::Email
                | Kind::PhoneNumber,
            ) => markdown_v2::Formattable::format(
                &to_formattable(&self.value),
                formatter,
                nesting,
            ),
            Some(Kind::TextLink(url)) => markdown_v2::Formattable::format(
                &link(to_formattable(&self.value), url.as_str()),
                formatter,
                nesting,
            ),
            Some(Kind::TextMention(user)) => markdown_v2::Formattable::format(
                &mention(to_formattable(&self.value), user.id),
                formatter,
                nesting,
            ),
        }
    }
}

impl html::Formattable for SemanticEntity {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match &self.kind {
            None
            | Some(
                Kind::Mention
                | Kind::Hashtag
                | Kind::Cashtag
                | Kind::BotCommand
                | Kind::Url
                | Kind::Email
                | Kind::PhoneNumber,
            ) => html::Formattable::format(
                &to_formattable(&self.value),
                formatter,
                nesting,
            ),
            Some(Kind::TextLink(url)) => html::Formattable::format(
                &link(to_formattable(&self.value), url.as_str()),
                formatter,
                nesting,
            ),
            Some(Kind::TextMention(user)) => html::Formattable::format(
                &mention(to_formattable(&self.value), user.id),
                formatter,
                nesting,
            ),
        }
    }
}

impl markdown_v2::Formattable for Entity {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match self {
            Self::Code(code) => markdown_v2::Formattable::format(
                &inline_code(code.as_str()),
                formatter,
                nesting,
            ),
            Self::Pre { language, value } => {
                let mut code = code_block(value.as_str());

                if let Some(language) = language {
                    code = code.language(language);
                }

                markdown_v2::Formattable::format(&code, formatter, nesting)
            }
            Self::Semantic(semantic) => {
                markdown_v2::Formattable::format(semantic, formatter, nesting)
            }
        }
    }
}

impl html::Formattable for Entity {
    fn format(
        &self,
        formatter: &mut std::fmt::Formatter,
        nesting: markup::Nesting,
    ) -> std::fmt::Result {
        match self {
            Self::Code(code) => html::Formattable::format(
                &inline_code(code.as_str()),
                formatter,
                nesting,
            ),
            Self::Pre { language, value } => {
                let mut code = code_block(value.as_str());

                if let Some(language) = language {
                    code = code.language(language);
                }

                html::Formattable::format(&code, formatter, nesting)
            }
            Self::Semantic(semantic) => {
                html::Formattable::format(semantic, formatter, nesting)
            }
        }
    }
}
