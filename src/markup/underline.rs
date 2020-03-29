use super::{html, markdown_v2, Formattable};
use std::fmt::{self, Formatter};

/// Formats text underlined. Can be created with [`underline`].
///
/// [`underline`]: ./fn.underline.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Underline<T>(T);

/// Formats text underlined.
pub fn underline<T: Formattable>(text: T) -> Underline<T> {
    Underline(text)
}

impl<T: Formattable> markdown_v2::Formattable for Underline<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("\r__")?;
        markdown_v2::Formattable::format(&self.0, formatter)?;
        formatter.write_str("\r__")
    }
}

impl<T: Formattable> html::Formattable for Underline<T> {
    fn format(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_str("<u>")?;
        html::Formattable::format(&self.0, formatter)?;
        formatter.write_str("</u>")
    }
}
