use crate::{IntoStyle, StyleAttribute};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Overflow {
    #[default]
    Auto,
    Visible,
    Scroll,
    Hidden,
}

impl IntoStyle for Overflow {
    #[inline]
    fn into_style(self) -> StyleAttribute {
        match self {
            Overflow::Auto => ("overflow", "auto").into_style(),
            Overflow::Visible => ("overflow", "visible").into_style(),
            Overflow::Scroll => ("overflow", "scroll").into_style(),
            Overflow::Hidden => ("overflow", "hidden").into_style(),
        }
    }
}
