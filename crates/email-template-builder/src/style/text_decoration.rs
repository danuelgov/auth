use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextDecoration {
    None,
    Underline,
    Overline,
    LineThrough,
}

impl IntoStyle for TextDecoration {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            TextDecoration::None => ("text-decoration", "none").into_style(),
            TextDecoration::Underline => ("text-decoration", "underline").into_style(),
            TextDecoration::Overline => ("text-decoration", "overline").into_style(),
            TextDecoration::LineThrough => ("text-decoration", "line-through").into_style(),
        }
    }
}
