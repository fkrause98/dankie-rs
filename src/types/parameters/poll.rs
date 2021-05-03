//! Types related to polls.

use super::{ParseMode, Text};
use is_macro::Is;
use serde::Serialize;
use std::convert::From;

/// Configures whether multiple answers are allowed in a poll.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
pub enum Answer {
    /// Only a single answer is allowed.
    Single,
    /// Multiple answers are allowed.
    Multiple,
}

/// Tells when the poll is automatically closed.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize, Is)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum AutoClose {
    /// Reflects the `open_period` field.
    OpenPeriod(u16),
    /// Reflects the `close_date` field.
    CloseDate(i64),
}

/// Represents a quiz.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Quiz {
    correct_option_id: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    explanation_parse_mode: Option<ParseMode>,
}

/// Represents a poll.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Serialize)]
pub struct Poll {
    allows_multiple_answers: bool,
}

/// Represents either a quiz or a poll.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize, Is)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Kind {
    /// Represents a quiz.
    Quiz(Quiz),
    /// Represents a poll.
    #[serde(rename = "regular")]
    Poll(Poll),
}

/// Represents a poll that will be sent to a user.
#[derive(Debug, PartialEq, Eq, Clone, Hash, Serialize)]
pub struct Any {
    #[serde(flatten)]
    kind: Kind,
    question: String,
    options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_closed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_anonymous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    auto_close: Option<AutoClose>,
}

impl Quiz {
    /// Constructs a new quiz.
    #[must_use]
    pub const fn new(correct_option_id: usize) -> Self {
        Self {
            correct_option_id,
            explanation: None,
            explanation_parse_mode: None,
        }
    }

    /// Sets the poll's explanation.
    /// Configures the `explanation` and `explanation_parse_mode` fields.
    pub fn explanation(mut self, explanation: impl Into<Text>) -> Self {
        let explanation = explanation.into();
        self.explanation = Some(explanation.text);
        self.explanation_parse_mode = explanation.parse_mode;
        self
    }
}

impl Poll {
    /// Constructs a new poll.
    #[must_use]
    pub fn new(answer: Answer) -> Self {
        Self {
            allows_multiple_answers: answer == Answer::Multiple,
        }
    }
}

impl Any {
    /// Constructs a poll.
    #[must_use]
    pub fn new(
        question: impl Into<String>,
        options: impl Into<Vec<String>>,
        kind: impl Into<Kind>,
    ) -> Self {
        Self {
            kind: kind.into(),
            question: question.into(),
            options: options.into(),
            is_closed: None,
            is_anonymous: None,
            auto_close: None,
        }
    }

    /// Configures if the poll is immediately closed.
    #[must_use]
    pub const fn is_immediately_closed(mut self, is_closed: bool) -> Self {
        self.is_closed = Some(is_closed);
        self
    }

    /// Comfigures if the poll is anonymous.
    #[must_use]
    pub const fn is_anonymous(mut self, is_anonymous: bool) -> Self {
        self.is_anonymous = Some(is_anonymous);
        self
    }

    /// Configures when the poll is automatically closed.
    /// Reflects the `open_period` and `close_date` parameters.
    #[must_use]
    pub const fn auto_close(mut self, auto_close: AutoClose) -> Self {
        self.auto_close = Some(auto_close);
        self
    }
}

impl From<Quiz> for Kind {
    fn from(quiz: Quiz) -> Self {
        Self::Quiz(quiz)
    }
}

impl From<Poll> for Kind {
    fn from(poll: Poll) -> Self {
        Self::Poll(poll)
    }
}
